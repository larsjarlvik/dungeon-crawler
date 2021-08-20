use crate::{
    config,
    engine::{self, pipelines::builders, texture},
};
use std::mem;
mod uniforms;

pub struct DeferredPipeline {
    render_pipeline: builders::Pipeline,
    sampler: wgpu::Sampler,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    render_bundle: Option<wgpu::RenderBundle>,
    pub depth_texture: texture::Texture,
    pub position_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub color_texture: texture::Texture,
}

impl DeferredPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let pipeline_builder = builders::PipelineBuilder::new(ctx, "deferred");

        let sampler = texture::Texture::create_sampler(ctx);
        let depth_texture = texture::Texture::create_depth_texture(&ctx, "deferred_depth_texture");
        let position_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_position_texture");
        let normal_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_normal_texture");
        let color_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_color_texture");

        let uniform_bind_group_layout = pipeline_builder.create_bindgroup_layout(
            0,
            "model_uniform_bind_group_layout",
            &[pipeline_builder.create_uniform_entry(0, wgpu::ShaderStage::FRAGMENT)],
        );

        let texture_bind_group_layout = pipeline_builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                pipeline_builder.create_texture_entry(0, wgpu::ShaderStage::FRAGMENT),
                pipeline_builder.create_texture_entry(1, wgpu::ShaderStage::FRAGMENT),
                pipeline_builder.create_texture_entry(2, wgpu::ShaderStage::FRAGMENT),
                pipeline_builder.create_texture_entry(3, wgpu::ShaderStage::FRAGMENT),
                pipeline_builder.create_sampler_entry(4, wgpu::ShaderStage::FRAGMENT),
            ],
        );

        let render_pipeline = pipeline_builder
            .with_shader(wgpu::ShaderSource::Wgsl(include_str!("deferred.wgsl").into()))
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            sampler,
            depth_texture,
            position_texture,
            normal_texture,
            color_texture,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            render_bundle: None,
        }
    }

    pub fn resize(&mut self, ctx: &engine::Context) {
        self.depth_texture = texture::Texture::create_depth_texture(&ctx, "deferred_depth_texture");
        self.position_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_position_texture");
        self.normal_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_normal_texture");
        self.color_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_color_texture");
    }

    pub fn update(&mut self, ctx: &engine::Context) {
        let uniforms = uniforms::Uniforms {
            light_pos: [10.0, 10.0, 10.0, 0.0],
            light_dir: [1.0, -1.0, 1.0, 0.0],
            light_color: [1.0, 1.0, 1.0, 0.0],
            light_ambient: [0.1, 0.1, 0.1, 0.0],
        };

        let builder = builders::RenderBundleBuilder::new(ctx, "deferred");
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<uniforms::Uniforms>() as u64);

        ctx.queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let texture_entries = &[
            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&self.depth_texture.view)),
            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&self.position_texture.view)),
            builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&self.normal_texture.view)),
            builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::TextureView(&self.color_texture.view)),
            builders::RenderBundleBuilder::create_entry(4, wgpu::BindingResource::Sampler(&self.sampler)),
        ];

        self.render_bundle = Some(
            builder
                .with_pipeline(&self.render_pipeline)
                .with_uniform_bind_group(&self.uniform_bind_group_layout, &uniform_buffer)
                .with_primitive(
                    builders::PrimitiveBuilder::new(ctx, "deferred")
                        .with_texture_bind_group(&self.texture_bind_group_layout, texture_entries)
                        .with_length(6),
                )
                .build(),
        );
    }

    pub fn render(&self, ctx: &engine::Context, view: &wgpu::TextureView) {
        if let Some(render_bundle) = &self.render_bundle {
            builders::RenderTargetBuilder::new(ctx, "deferred")
                .with_color_attachment(view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
                .execute_bundles(vec![&render_bundle]);
        }
    }
}
