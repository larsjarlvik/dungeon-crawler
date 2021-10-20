mod model;
mod pipeline_display;
mod pipeline_shadow;
mod uniforms;
use crate::{
    config,
    engine::{
        self,
        pipelines::{self},
    },
    world::*,
};
use cgmath::*;
pub use model::Model;
use specs::{Join, WorldExt};
use std::convert::TryInto;
pub use uniforms::Uniforms;

pub struct ModelPipeline {
    pub display: pipeline_display::PipelineDisplay,
    pub shadows: pipeline_shadow::PipelineShadow,
}

impl ModelPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        Self {
            shadows: pipeline_shadow::PipelineShadow::new(ctx),
            display: pipeline_display::PipelineDisplay::new(ctx),
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &specs::World, target: &pipelines::DeferredPipeline) {
        let models = components.read_storage::<components::Model>();
        let render = components.read_storage::<components::Render>();
        let shadow = components.read_storage::<components::Shadow>();
        let transform = components.read_storage::<components::Transform>();
        let animation = components.read_storage::<components::Animations>();
        let time = components.read_resource::<resources::Time>();
        let camera = components.read_resource::<resources::Camera>();

        let mut bundles = vec![];
        let mut shadow_bundles = vec![];

        for (model, animation, render, shadow, transform) in (&models, (&animation).maybe(), &render, (&shadow).maybe(), &transform).join()
        {
            let model_matrix = transform.to_matrix(time.last_frame);

            if render.cull_frustum {
                let transformed_bb = model.model.bounding_box.transform(model_matrix.into());
                if !camera.frustum.test_bounding_box(&transformed_bb) {
                    continue;
                }
            }

            let joint_transforms = get_joint_transforms(&model, &animation);
            ctx.queue.write_buffer(
                &model.model.display_uniform_buffer,
                self.display.uniform_bind_group_layout.index as u64,
                bytemuck::cast_slice(&[Uniforms {
                    view_proj: camera.view_proj.into(),
                    model: model_matrix.into(),
                    joint_transforms: joint_transforms.clone().try_into().unwrap(),
                    is_animated: animation.is_some() as u32,
                }]),
            );

            bundles.push(&model.model.display_render_bundle);

            if shadow.is_some() {
                ctx.queue.write_buffer(
                    &model.model.shadow_uniform_buffer,
                    self.shadows.uniform_bind_group_layout.index as u64,
                    bytemuck::cast_slice(&[Uniforms {
                        view_proj: camera.get_shadow_matrix().into(),
                        model: model_matrix.into(),
                        joint_transforms: joint_transforms.clone().try_into().unwrap(),
                        is_animated: animation.is_some() as u32,
                    }]),
                );
                shadow_bundles.push(&model.model.shadow_render_bundle);
            }
        }

        self.display.execute_bundles(ctx, bundles, target);
        self.shadows.execute_bundles(ctx, shadow_bundles, target);
    }
}

fn get_joint_transforms(model: &components::Model, animation: &Option<&components::Animations>) -> Vec<[[f32; 4]; 4]> {
    if let Some(animation) = animation {
        let mut joint_transforms = vec![Matrix4::identity(); config::MAX_JOINT_COUNT];

        animation.channels.iter().for_each(|(_, channel)| {
            let blend_factor = channel.get_blend_factor();

            if blend_factor < 1.0 {
                if let Some(prev) = &channel.prev {
                    animate(model, &prev, &mut joint_transforms, 1.0 - blend_factor);
                }
            }

            animate(model, &channel.current, &mut joint_transforms, blend_factor);
        });

        joint_transforms.iter().map(|jm| jm.clone().into()).collect()
    } else {
        vec![[[0.0; 4]; 4]; config::MAX_JOINT_COUNT]
    }
}

fn animate(
    model: &components::Model,
    animation: &components::animation::Animation,
    joint_matrices: &mut Vec<Matrix4<f32>>,
    blend_factor: f32,
) {
    let mut nodes = model.nodes.clone();
    let model_animation = model
        .animations
        .get(&animation.name)
        .expect(format!("Could not find animation: {}", &animation.name).as_str());

    if model_animation.animate_nodes(&mut nodes, animation.elapsed % model_animation.total_time) {
        for (index, parent_index) in &model.depth_first_taversal_indices {
            let parent_transform = parent_index
                .map(|id| {
                    let parent = &nodes[id];
                    parent.global_transform_matrix
                })
                .or(Matrix4::identity().into());

            if let Some(matrix) = parent_transform {
                let node = &mut nodes[*index];
                node.apply_transform(matrix);
            }
        }

        let transforms: Vec<(usize, Matrix4<f32>)> = nodes
            .iter()
            .filter(|n| n.skin_index.is_some())
            .map(|n| {
                (
                    n.skin_index.unwrap(),
                    n.global_transform_matrix.invert().expect("Transform matrix should be invertible"),
                )
            })
            .collect();

        for (s_index, inverse_transform) in transforms {
            model.skins[s_index].joints.iter().enumerate().for_each(|(j_index, joint)| {
                joint_matrices[j_index] = joint_matrices[j_index].lerp(
                    inverse_transform * nodes[joint.node_id].global_transform_matrix * joint.inverse_bind_matrix,
                    blend_factor,
                );
            });
        }
    }
}
