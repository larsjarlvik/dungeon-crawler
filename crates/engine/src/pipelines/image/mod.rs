mod uniforms;
use crate::{config, pipelines::builders, texture, Context};
use std::mem;

use self::uniforms::Uniforms;
pub mod context;

pub struct ImagePipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    sampler: wgpu::Sampler,
}

impl ImagePipeline {
    pub fn new(ctx: &Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "asset");
        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT, true),
                builder.create_sampler_entry(1, wgpu::ShaderStages::FRAGMENT, false),
            ],
        );

        let render_pipeline = builder
            .with_shader("shaders/asset.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
            .with_blend(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
                alpha: wgpu::BlendComponent::OVER,
            })
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        let builder = builders::RenderBundleBuilder::new(ctx, "asset");
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<uniforms::Uniforms>() as u64);
        let sampler = texture::Texture::create_sampler(ctx, wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear, None);

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            uniform_buffer,
            texture_bind_group_layout,
            sampler,
        }
    }

    pub fn render(&self, ctx: &mut Context, target: &wgpu::TextureView) {
        for (id, data) in ctx.images.queue.iter() {
            ctx.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&[Uniforms {
                    position: data.position.into(),
                    size: data.size.into(),
                    viewport_size: [ctx.viewport.width as f32, ctx.viewport.height as f32],
                    background: data.background.into(),
                    has_image: data.has_image as u32,
                }]),
            );

            let mut primitive_builder = builders::PrimitiveBuilder::new(ctx, "asset").with_length(4);
            if let Some(id) = id {
                let asset = ctx.images.textures.get(id).expect("Image not found!");
                primitive_builder = primitive_builder.with_texture_bind_group(
                    &self.texture_bind_group_layout,
                    &[
                        builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&asset.view)),
                        builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&self.sampler)),
                    ],
                );
            }

            let bundle = builders::RenderBundleBuilder::new(ctx, "asset")
                .with_pipeline(&self.render_pipeline)
                .with_uniform_bind_group(&self.uniform_bind_group_layout, &self.uniform_buffer)
                .with_primitive(primitive_builder)
                .build();

            builders::RenderTargetBuilder::new(ctx, "particle")
                .with_color_attachment(&target, wgpu::LoadOp::Load)
                .execute_bundles(vec![&bundle]);
        }

        ctx.images.queue.clear();
    }
}
