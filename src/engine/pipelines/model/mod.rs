mod model;
mod pipeline_display;
mod pipeline_shadow;
mod uniforms;
use crate::{
    config,
    engine::{
        self,
        model::GltfModelNodes,
        pipelines::{self},
    },
    world::*,
};
use cgmath::*;
pub use model::Model;
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

    pub fn render(&self, ctx: &engine::Context, components: &mut bevy_ecs::world::World, target: &pipelines::DeferredPipeline) {
        let alpha = { components.get_resource::<resources::Time>().unwrap().alpha };
        let (frustum, view_proj, shadow_matrix) = {
            let camera = components.get_resource::<resources::Camera>().unwrap();
            (camera.frustum, camera.view_proj, camera.get_shadow_matrix())
        };

        let mut bundles = vec![];
        let mut shadow_bundles = vec![];

        for (model_instance, animation, render, shadow, transform) in components
            .query::<(
                &components::Model,
                Option<&components::Animations>,
                &components::Render,
                Option<&components::Shadow>,
                &components::Transform,
            )>()
            .iter(components)
        {
            let model_matrix = transform.to_matrix(alpha);
            let model = ctx
                .model_instances
                .get(&model_instance.key)
                .expect(format!("Could not find model \"{}\"!", model_instance.key).as_str());

            if render.cull_frustum {
                let transformed_bb = model.model.bounding_box.transform(model_matrix.into());
                if !frustum.test_bounding_box(&transformed_bb) {
                    continue;
                }
            }

            let joint_transforms = get_joint_transforms(&model.nodes, &animation);
            let inv_model = model_matrix.invert().unwrap().transpose().into();

            ctx.queue.write_buffer(
                &model.model.display_uniform_buffer,
                self.display.uniform_bind_group_layout.index as u64,
                bytemuck::cast_slice(&[Uniforms {
                    view_proj: view_proj.into(),
                    model: model_matrix.into(),
                    inv_model,
                    joint_transforms: joint_transforms.clone().try_into().unwrap(),
                    highlight: model_instance.highlight,
                    is_animated: animation.is_some() as u32,
                }]),
            );

            bundles.push(&model.model.display_render_bundle);

            if shadow.is_some() {
                ctx.queue.write_buffer(
                    &model.model.shadow_uniform_buffer,
                    self.shadows.uniform_bind_group_layout.index as u64,
                    bytemuck::cast_slice(&[Uniforms {
                        view_proj: shadow_matrix.into(),
                        model: model_matrix.into(),
                        inv_model,
                        joint_transforms: joint_transforms.clone().try_into().unwrap(),
                        highlight: model_instance.highlight,
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

fn get_joint_transforms(nodes: &GltfModelNodes, animation: &Option<&components::Animations>) -> Vec<[[f32; 4]; 4]> {
    if let Some(animation) = animation {
        let mut joint_transforms = vec![Matrix4::identity(); config::MAX_JOINT_COUNT];

        animation.channels.iter().for_each(|(_, channel)| {
            let blend_factor = channel.get_blend_factor();

            if blend_factor < 1.0 {
                if let Some(prev) = &channel.prev {
                    animate(nodes, &prev, &mut joint_transforms, 1.0 - blend_factor);
                }
            }

            animate(nodes, &channel.current, &mut joint_transforms, blend_factor);
        });

        joint_transforms.iter().map(|jm| jm.clone().into()).collect()
    } else {
        vec![[[0.0; 4]; 4]; config::MAX_JOINT_COUNT]
    }
}

fn animate(
    nodes: &GltfModelNodes,
    animation: &components::animation::Animation,
    joint_matrices: &mut Vec<Matrix4<f32>>,
    blend_factor: f32,
) {
    let mut nodes = nodes.clone();
    let model_animation = nodes
        .animations
        .get(&animation.name)
        .expect(format!("Could not find animation: {}", &animation.name).as_str());

    let time = if animation.repeat {
        animation.elapsed % model_animation.total_time
    } else {
        animation.elapsed.min(model_animation.total_time)
    };

    if model_animation.animate_nodes(&mut nodes.nodes, time) {
        for (index, parent_index) in &nodes.depth_first_taversal_indices {
            let parent_transform = parent_index
                .map(|id| {
                    let parent = &nodes.nodes[id];
                    parent.global_transform_matrix
                })
                .or(Matrix4::identity().into());

            if let Some(matrix) = parent_transform {
                let node = &mut nodes.nodes[*index];
                node.apply_transform(matrix);
            }
        }

        let transforms: Vec<(usize, Matrix4<f32>)> = nodes
            .nodes
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
            nodes.skins[s_index].joints.iter().enumerate().for_each(|(j_index, joint)| {
                joint_matrices[j_index] = joint_matrices[j_index].lerp(
                    inverse_transform * nodes.nodes[joint.node_id].global_transform_matrix * joint.inverse_bind_matrix,
                    blend_factor,
                );
            });
        }
    }
}
