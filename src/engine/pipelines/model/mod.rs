mod model;
mod uniforms;
use crate::{
    config,
    engine::{
        self,
        pipelines::{self, builders},
        texture,
    },
    world::*,
};
use cgmath::*;
pub use model::Model;
use specs::{Join, WorldExt};
use std::convert::TryInto;
pub use uniforms::Uniforms;

pub struct ModelPipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    primitive_uniform_bind_group_layout: builders::MappedBindGroupLayout,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    sampler: wgpu::Sampler,
}

impl ModelPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "model");
        let sampler = texture::Texture::create_sampler(ctx);

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "model_uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX)],
        );

        let primitive_uniform_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "model_uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            2,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT),
                builder.create_texture_entry(1, wgpu::ShaderStages::FRAGMENT),
                builder.create_texture_entry(2, wgpu::ShaderStages::FRAGMENT),
                builder.create_sampler_entry(3, wgpu::ShaderStages::FRAGMENT),
            ],
        );

        let render_pipeline = builder
            .with_shader("shaders/model.wgsl")
            .with_color_targets(vec![
                config::COLOR_TEXTURE_FORMAT,
                config::COLOR_TEXTURE_FORMAT,
                config::COLOR_TEXTURE_FORMAT,
            ])
            .with_depth_target(config::DEPTH_FORMAT)
            .with_buffer_layouts(vec![engine::model::Vertex::desc()])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&primitive_uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            primitive_uniform_bind_group_layout,
            texture_bind_group_layout,
            sampler,
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &specs::World, target: &pipelines::DeferredPipeline) {
        let models = components.read_storage::<components::Model>();
        let render = components.read_storage::<components::Render>();
        let transform = components.read_storage::<components::Transform>();
        let animation = components.read_storage::<components::Animations>();
        let time = components.read_resource::<resources::Time>();
        let camera = components.read_resource::<resources::Camera>();

        let bundles = (&models, (&animation).maybe(), &render, &transform)
            .join()
            .filter_map(|(model, animation, render, transform)| {
                let model_matrix = transform.to_matrix(time.last_frame);

                if render.cull_frustum
                    && !camera
                        .frustum
                        .test_bounding_box(&model.model.bounding_box.transform(model_matrix.into()))
                {
                    return None;
                }

                let joint_transforms = get_joint_transforms(&model, &animation);
                ctx.queue.write_buffer(
                    &model.model.uniform_buffer,
                    self.uniform_bind_group_layout.index as u64,
                    bytemuck::cast_slice(&[Uniforms {
                        view_proj: camera.view_proj.into(),
                        model: model_matrix.into(),
                        joint_transforms: joint_transforms.try_into().unwrap(),
                        is_animated: animation.is_some() as u32,
                    }]),
                );

                Some(&model.model.render_bundle)
            })
            .collect();

        builders::RenderTargetBuilder::new(ctx, "model")
            .with_color_attachment(&target.normal_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.color_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.orm_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_depth_attachment(&target.depth_texture.view, wgpu::LoadOp::Clear(1.0))
            .execute_bundles(bundles);
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
