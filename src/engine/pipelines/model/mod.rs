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
use specs::{Join, WorldExt};
use std::mem;
pub use uniforms::Uniforms;

pub struct Model {
    pub uniform_buffer: wgpu::Buffer,
    pub render_bundle: wgpu::RenderBundle,
}

pub struct ModelPipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
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
            &[builder.create_uniform_entry(0, wgpu::ShaderStage::VERTEX)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStage::FRAGMENT),
                builder.create_texture_entry(1, wgpu::ShaderStage::FRAGMENT),
                builder.create_texture_entry(2, wgpu::ShaderStage::FRAGMENT),
                builder.create_sampler_entry(3, wgpu::ShaderStage::FRAGMENT),
            ],
        );

        let render_pipeline = builder
            .with_shader(wgpu::ShaderSource::Wgsl(include_str!("model.wgsl").into()))
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT, config::COLOR_TEXTURE_FORMAT])
            .with_depth_target(config::DEPTH_FORMAT)
            .with_buffer_layouts(vec![engine::model::Vertex::desc()])
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

    pub fn gltf(&self, ctx: &engine::Context, model: &engine::model::GltfModel, mesh_name: &str) -> Model {
        let mesh = model.get_mesh_by_name(mesh_name);

        let builder = builders::RenderBundleBuilder::new(ctx, mesh_name);
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<Uniforms>() as u64);

        let mut builder = builder
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &uniform_buffer);

        for primitive in &mesh.primitives {
            let material = model.get_material(primitive.material);
            let texture_entries = &[
                builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&material.base_color_texture.view)),
                builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&material.normal_texture.view)),
                builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&material.orm_texture.view)),
                builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::Sampler(&self.sampler)),
            ];

            builder = builder.with_primitive(
                builders::PrimitiveBuilder::new(ctx, mesh_name)
                    .with_texture_bind_group(&self.texture_bind_group_layout, texture_entries)
                    .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
                    .with_indices(bytemuck::cast_slice(&primitive.indices.as_slice()))
                    .with_length(primitive.length),
            );
        }

        let render_bundle = builder.build();
        Model {
            uniform_buffer,
            render_bundle,
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &specs::World, target: &pipelines::DeferredPipeline) {
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

        builders::RenderTargetBuilder::new(ctx, "model")
            .with_color_attachment(&target.normal_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.color_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_depth_attachment(&target.depth_texture.view, wgpu::LoadOp::Clear(1.0))
            .execute_bundles(bundles);
    }
}