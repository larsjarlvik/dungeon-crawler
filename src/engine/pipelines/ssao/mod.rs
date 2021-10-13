mod uniforms;
use crate::{
    config,
    engine::{self, pipelines::builders, texture},
    world::resources,
};
use cgmath::*;
use rand::Rng;
use specs::WorldExt;
pub use uniforms::Uniforms;

pub struct SsaoPipeline {
    render_bundle: wgpu::RenderBundle,
    uniform_buffer: wgpu::Buffer,
    ssao_kernel: [[f32; 4]; config::SSAO_KERNEL_SIZE],
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    sampler: wgpu::Sampler,
}

impl SsaoPipeline {
    pub fn new(ctx: &engine::Context, deferred: &engine::pipelines::DeferredPipeline) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "ssao");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
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
            .with_shader("shaders/ssao.wgsl")
            .with_color_targets(vec![wgpu::TextureFormat::R8Unorm])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        let mut ssao_kernel = [[0.0, 0.0, 0.0, 0.0]; config::SSAO_KERNEL_SIZE];
        let mut rng = rand::thread_rng();
        for i in 0..config::SSAO_KERNEL_SIZE {
            let mut sample = vec4(
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>(),
                0.0,
            )
            .normalize();

            sample *= rng.gen::<f32>();
            let scale = vec1(0.4f32)
                .lerp(vec1(1.0), (i as f32 / config::SSAO_KERNEL_SIZE as f32).powf(2.0))
                .x;
            sample *= scale;
            ssao_kernel[i] = sample.into();
        }

        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "ssao");
        let uniform_buffer = render_bundle_builder.create_uniform_buffer_init(bytemuck::cast_slice(&[Uniforms {
            viewport: [
                ctx.viewport.get_render_width() / 2.0,
                ctx.viewport.get_render_height() / 2.0,
                0.0,
                0.0,
            ],
            ssao_kernel,
        }]));

        let sampler = texture::Texture::create_sampler(ctx, wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Nearest);
        let texture_entries = [
            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&deferred.depth_texture.view)),
            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&sampler)),
        ];

        let render_bundle = render_bundle_builder
            .with_pipeline(&render_pipeline)
            .with_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer)
            .with_primitive(
                builders::PrimitiveBuilder::new(ctx, "ssao")
                    .with_texture_bind_group(&texture_bind_group_layout, &texture_entries)
                    .with_length(4),
            )
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            render_bundle,
            uniform_buffer,
            ssao_kernel,
            sampler,
        }
    }

    pub fn update(&mut self, ctx: &engine::Context, deferred: &engine::pipelines::DeferredPipeline, components: &specs::World) {
        let camera = components.read_resource::<resources::Camera>();

        ctx.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[Uniforms {
                viewport: [
                    ctx.viewport.get_render_width(),
                    ctx.viewport.get_render_height(),
                    camera.znear,
                    camera.zfar,
                ],
                ssao_kernel: self.ssao_kernel,
            }]),
        );

        let texture_entries = [
            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&deferred.depth_texture.view)),
            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&self.sampler)),
        ];

        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "ssao");
        self.render_bundle = render_bundle_builder
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &self.uniform_buffer)
            .with_primitive(
                builders::PrimitiveBuilder::new(ctx, "ssao")
                    .with_texture_bind_group(&self.texture_bind_group_layout, &texture_entries)
                    .with_length(4),
            )
            .build();
    }

    pub fn render(&self, ctx: &engine::Context, target: &wgpu::TextureView) {
        builders::RenderTargetBuilder::new(ctx, "ssao")
            .with_color_attachment(&target, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .execute_bundles(vec![&self.render_bundle]);
    }
}
