mod initializer;
mod pipeline_display;
mod pipeline_shadow;
mod uniforms;
use crate::{
    config,
    ecs::{components, resources},
    interpolated_value::Interpolate,
    model::GltfModelNodes,
    Context,
};
use bevy_ecs::prelude::World;
use cgmath::*;
pub use initializer::Model;
use std::convert::TryInto;
pub use uniforms::Uniforms;

use self::uniforms::EnvironmentUniforms;

pub struct ModelPipeline {
    pub display: pipeline_display::PipelineDisplay,
    pub shadows: pipeline_shadow::PipelineShadow,
}

impl ModelPipeline {
    pub fn new(ctx: &Context) -> Self {
        Self {
            shadows: pipeline_shadow::PipelineShadow::new(ctx),
            display: pipeline_display::PipelineDisplay::new(ctx),
        }
    }

    pub fn render(
        &self,
        ctx: &Context,
        components: &mut bevy_ecs::world::World,
        target: &wgpu::TextureView,
        depth_target: &wgpu::TextureView,
        shadow_target: &wgpu::TextureView,
    ) {
        let alpha = { components.get_resource::<resources::Time>().unwrap().alpha };
        let (frustum, view_proj, shadow_matrix, eye, eye_target) = {
            let camera = components.get_resource::<resources::Camera>().unwrap();
            (
                camera.frustum,
                camera.view_proj,
                camera.get_shadow_matrix(),
                camera.eye,
                camera.target,
            )
        };

        let mut bundles = vec![];
        let mut shadow_bundles = vec![];

        let (lights_count, lights) = self.get_lights(ctx, components);

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
            let model = model_instance.get_model(ctx);

            if render.cull_frustum {
                let transformed_bb = model.model.bounding_box.transform(model_matrix);
                if !frustum.test_bounding_box(&transformed_bb) {
                    continue;
                }
            }

            let joint_transforms = get_joint_transforms(&model.nodes, &animation);
            let inv_model = model_matrix.invert().unwrap().transpose().into();

            ctx.queue.write_buffer(
                &model.model.display_uniform_buffer,
                0,
                bytemuck::cast_slice(&[Uniforms {
                    view_proj: view_proj.into(),
                    model: model_matrix.into(),
                    inv_model,
                    joint_transforms: joint_transforms.clone().try_into().unwrap(),
                    is_animated: animation.is_some() as u32,
                }]),
            );

            ctx.queue.write_buffer(
                &model.model.display_environment_uniform_buffer,
                0,
                bytemuck::cast_slice(&[EnvironmentUniforms {
                    eye_pos: eye.to_vec().extend(0.0).into(),
                    target: eye_target.extend(0.0).into(),
                    lights,
                    lights_count,
                    contrast: ctx.settings.contrast,
                }]),
            );

            bundles.push(&model.model.display_render_bundle);

            if shadow.is_some() {
                ctx.queue.write_buffer(
                    &model.model.shadow_uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[Uniforms {
                        view_proj: shadow_matrix.into(),
                        model: model_matrix.into(),
                        inv_model,
                        joint_transforms: joint_transforms.clone().try_into().unwrap(),
                        is_animated: animation.is_some() as u32,
                    }]),
                );
                shadow_bundles.push(&model.model.shadow_render_bundle);
            }
        }

        self.display.execute_bundles(ctx, bundles, target, depth_target);
        self.shadows.execute_bundles(ctx, shadow_bundles, shadow_target);
    }

    fn get_lights(&self, ctx: &Context, components: &mut World) -> (i32, [uniforms::LightUniforms; 16]) {
        let alpha = {
            let time = components.get_resource::<resources::Time>().unwrap();
            time.alpha
        };
        let (frustum, target) = {
            let camera = components.get_resource::<resources::Camera>().unwrap();
            (camera.frustum, camera.target)
        };

        let mut lights: [uniforms::LightUniforms; 16] = Default::default();

        let mut visible_lights: Vec<(&components::Light, &components::Transform)> = components
            .query::<(&components::Light, &components::Transform)>()
            .iter(components)
            .filter(|(light, transform)| {
                if let Some(bounding_sphere) = &light.bounding_sphere {
                    frustum.test_bounding_sphere(&bounding_sphere.transform(transform.to_matrix(alpha)))
                } else {
                    true
                }
            })
            .collect();

        visible_lights.sort_by(|a, b| {
            a.1.translation
                .get(alpha)
                .distance(target)
                .partial_cmp(&b.1.translation.get(alpha).distance(target))
                .unwrap()
        });

        for (i, (light, transform)) in visible_lights.iter().enumerate() {
            let radius = if let Some(radius) = light.radius { radius } else { 0.0 };
            if i >= lights.len() {
                break;
            }

            lights[i] = uniforms::LightUniforms {
                position: (transform.translation.get(alpha) + light.offset.get(alpha)).into(),
                radius,
                color: (light.color * light.base_intensity * light.intensity.get(alpha)).into(),
                bloom: light.bloom * ctx.settings.bloom,
            };
        }

        (visible_lights.len() as i32, lights)
    }
}

fn get_joint_transforms(model_nodes: &GltfModelNodes, animation: &Option<&components::Animations>) -> Vec<[[f32; 4]; 4]> {
    if let Some(animation) = animation {
        let mut joint_transforms = vec![Matrix4::identity(); config::MAX_JOINT_COUNT];
        let mut nodes = model_nodes.nodes.clone();

        animation.channels.iter().for_each(|(_, channel)| {
            for (index, animation) in channel.queue.iter().enumerate() {
                let blend_factor = channel.get_blend_factor(index);
                if blend_factor > 0.01 {
                    let cur_model_animation = model_nodes
                        .animations
                        .get(&animation.name)
                        .unwrap_or_else(|| panic!("Could not find animation: {}", &animation.name));

                    cur_model_animation.animate_nodes(&mut nodes, animation.elapsed, blend_factor);
                }
            }

            for (index, parent_index) in &model_nodes.depth_first_taversal_indices {
                let parent_transform = parent_index
                    .map(|id| {
                        let parent = &nodes[id];
                        parent.global_transform_matrix
                    })
                    .or_else(|| Some(Matrix4::identity()));

                let node = &mut nodes[*index];
                node.apply_transform(parent_transform);
            }

            for node in nodes.iter() {
                let inverse_transform = node
                    .global_transform_matrix
                    .invert()
                    .expect("Transform matrix should be invertible");

                let skin_index = if let Some(skin_index) = node.skin_index { skin_index } else { 0 };

                model_nodes.skins[skin_index]
                    .joints
                    .iter()
                    .enumerate()
                    .for_each(|(j_index, joint)| {
                        joint_transforms[j_index] =
                            inverse_transform * nodes[joint.node_id].global_transform_matrix * joint.inverse_bind_matrix;
                    });
            }
        });

        joint_transforms.iter().map(|jm| (*jm).into()).collect()
    } else {
        vec![[[0.0; 4]; 4]; config::MAX_JOINT_COUNT]
    }
}
