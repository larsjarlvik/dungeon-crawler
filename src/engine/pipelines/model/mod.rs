mod uniforms;

use crate::{
    config,
    engine::{
        self,
        pipelines::{self, pipeline_builder::PipelineBuilder},
    },
    world::*,
};
use specs::{Join, WorldExt};
use std::mem;
pub use uniforms::Uniforms;

use super::render_bundle_builder::RenderBundleBuilder;

pub struct Model {
    pub uniform_buffer: wgpu::Buffer,
    pub render_bundle: wgpu::RenderBundle,
}

pub struct ModelPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub sampler: wgpu::Sampler,
}

impl ModelPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = PipelineBuilder::new(&ctx);

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            "model_uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStage::VERTEX)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStage::FRAGMENT),
                builder.create_texture_entry(1, wgpu::ShaderStage::FRAGMENT),
                builder.create_texture_entry(2, wgpu::ShaderStage::FRAGMENT),
                builder.create_sampler_entry(3, wgpu::ShaderStage::FRAGMENT),
            ],
        );

        let sampler = builder.create_sampler();
        let render_pipeline = builder
            .with_shader(wgpu::ShaderSource::Wgsl(include_str!("model.wgsl").into()))
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            sampler,
        }
    }

    pub fn gltf(&self, ctx: &engine::Context, model: &engine::model::GltfModel, mesh: &str) -> Model {
        let mesh = model.get_mesh_by_name(mesh);
        let primitive = mesh.primitives.first().unwrap();
        let material = model.get_material(primitive.material);

        let builder = RenderBundleBuilder::new(ctx);
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<Uniforms>() as u64);
        let texture_entries = &[
            RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&material.base_color_texture.view)),
            RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&material.normal_texture.view)),
            RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&material.orm_texture.view)),
            RenderBundleBuilder::create_entry(3, wgpu::BindingResource::Sampler(&self.sampler)),
        ];

        let render_bundle = builder
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &uniform_buffer)
            .with_texture_bind_group(&self.texture_bind_group_layout, texture_entries)
            .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
            .with_indices(bytemuck::cast_slice(&primitive.indices.as_slice()))
            .with_length(primitive.length)
            .build();

        Model {
            uniform_buffer,
            render_bundle,
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &specs::World, view: &wgpu::TextureView) {
        let models = components.read_storage::<components::Model>();
        let render = components.read_storage::<components::Render>();
        let mut bundles = vec![];

        for (model, render) in (&models, &render).join() {
            let uniforms = pipelines::model::Uniforms {
                view_proj: render.view_proj.into(),
                model: render.model_matrix.into(),
            };

            ctx.queue.write_buffer(&model.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            bundles.push(&model.render_bundle);
        }

        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("model_encoder"),
        });

        encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("model_render_pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(config::CLEAR_COLOR),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &ctx.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            })
            .execute_bundles(bundles.into_iter());

        ctx.queue.submit(std::iter::once(encoder.finish()));
    }
}
