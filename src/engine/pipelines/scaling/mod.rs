mod uniforms;
use crate::{
    config,
    engine::{self, pipelines::builders, texture},
};
pub use uniforms::Uniforms;

pub struct ScalingPipeline {
    render_bundle: wgpu::RenderBundle,
    render_pipeline: builders::Pipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    sampler: wgpu::Sampler,
    pub texture: texture::Texture,
}

impl ScalingPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "scaling");
        let sampler = texture::Texture::create_sampler(ctx, wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear);

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::FRAGMENT)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT),
                builder.create_sampler_entry(1, wgpu::ShaderStages::FRAGMENT),
            ],
        );

        let render_pipeline = builder
            .with_shader("shaders/scaling.wgsl")
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "scaling");
        let texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "texture");
        let uniform_buffer = render_bundle_builder.create_uniform_buffer_init(bytemuck::cast_slice(&[Uniforms {
            width: ctx.viewport.width as f32,
            height: ctx.viewport.height as f32,
        }]));

        let render_bundle = render_bundle_builder
            .with_pipeline(&render_pipeline)
            .with_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer)
            .with_primitive(
                builders::PrimitiveBuilder::new(ctx, "scaling")
                    .with_texture_bind_group(
                        &texture_bind_group_layout,
                        &[
                            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&texture.view)),
                            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&sampler)),
                        ],
                    )
                    .with_length(4),
            )
            .build();

        Self {
            texture,
            render_pipeline,
            render_bundle,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            sampler,
            uniform_buffer,
        }
    }

    pub fn resize(&mut self, ctx: &engine::Context) {
        self.texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "texture");
        ctx.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[Uniforms {
                width: ctx.viewport.width as f32,
                height: ctx.viewport.height as f32,
            }]),
        );

        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "scaling");
        self.render_bundle = render_bundle_builder
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &self.uniform_buffer)
            .with_primitive(
                builders::PrimitiveBuilder::new(ctx, "scaling")
                    .with_texture_bind_group(
                        &self.texture_bind_group_layout,
                        &[
                            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&self.texture.view)),
                            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&self.sampler)),
                        ],
                    )
                    .with_length(4),
            )
            .build();
    }

    pub fn render(&self, ctx: &engine::Context, target: &wgpu::TextureView) {
        builders::RenderTargetBuilder::new(ctx, "scaling")
            .with_color_attachment(&target, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .execute_bundles(vec![&self.render_bundle]);
    }
}
