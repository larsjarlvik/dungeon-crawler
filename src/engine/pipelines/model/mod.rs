mod model;
mod uniforms;
use std::convert::TryInto;

pub use model::Model;

use crate::{
    config,
    engine::{
        self,
        pipelines::{self, builders},
        texture,
    },
    world::*,
};
use specs::{Join, WorldExt};
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
        let mut bundles = vec![];

        for (model, render) in (&models, &render).join() {
            let mut joint_transforms = vec![[[0.0; 4]; 4]; 20];
            for (_, skin) in model.skins.iter().enumerate() {
                for (index, joint) in skin.joints.iter().take(20).enumerate() {
                    let joint_matrix = joint.matrix;
                    joint_transforms[index] = joint_matrix.into();
                }
            }

            ctx.queue.write_buffer(
                &model.model.uniform_buffer,
                self.uniform_bind_group_layout.index as u64,
                bytemuck::cast_slice(&[Uniforms {
                    view_proj: render.view_proj.into(),
                    model: render.model_matrix.into(),
                    joint_transforms: joint_transforms.try_into().unwrap(),
                    is_animated: (model.skins.len() > 0) as u32,
                }]),
            );

            bundles.push(&model.model.render_bundle);
        }

        builders::RenderTargetBuilder::new(ctx, "model")
            .with_color_attachment(&target.normal_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.color_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.orm_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_depth_attachment(&target.depth_texture.view, wgpu::LoadOp::Clear(1.0))
            .execute_bundles(bundles);
    }
}
