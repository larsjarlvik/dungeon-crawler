use crate::{
    config,
    engine::{self, pipelines::builders, texture},
    world::{components, resources},
};
use cgmath::*;
use specs::{Join, WorldExt};
use std::mem;

mod uniforms;

pub struct DeferredPipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    render_bundle: Option<wgpu::RenderBundle>,
    uniform_buffer: wgpu::Buffer,
    pub depth_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub color_texture: texture::Texture,
}

impl DeferredPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let pipeline_builder = builders::PipelineBuilder::new(ctx, "deferred");

        let depth_texture = texture::Texture::create_depth_texture(&ctx, "deferred_depth_texture");
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
            ],
        );

        let render_pipeline = pipeline_builder
            .with_shader(wgpu::ShaderSource::Wgsl(include_str!("deferred.wgsl").into()))
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        let builder = builders::RenderBundleBuilder::new(ctx, "deferred");
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<uniforms::Uniforms>() as u64);

        Self {
            render_pipeline,
            depth_texture,
            normal_texture,
            color_texture,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            uniform_buffer,
            render_bundle: None,
        }
    }

    pub fn resize(&mut self, ctx: &engine::Context) {
        self.depth_texture = texture::Texture::create_depth_texture(&ctx, "deferred_depth_texture");
        self.normal_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_normal_texture");
        self.color_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, "deferred_color_texture");
    }

    pub fn update(&mut self, ctx: &engine::Context, components: &specs::World) {
        let light_sources = components.read_storage::<components::Light>();
        let positions = components.read_storage::<components::Position>();
        let mut lights: [uniforms::LightUniforms; 10] = Default::default();

        for (i, (light, position)) in (&light_sources, &positions).join().enumerate() {
            let radius = if let Some(radius) = light.radius { radius } else { 0.0 };

            lights[i] = uniforms::LightUniforms {
                position: position.0.into(),
                radius,
                color: light.color.extend(0.0).into(),
            };
        }

        let camera = components.read_resource::<resources::Camera>();
        let uniforms = uniforms::Uniforms {
            inv_view_proj: camera.view_proj.invert().unwrap().into(),
            eye_pos: camera.eye.to_vec().extend(0.0).into(),
            viewport_size: [ctx.viewport.width as f32, ctx.viewport.height as f32, 0.0, 0.0],
            lights,
            lights_count: lights.len() as i32,
        };

        ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let texture_entries = &[
            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&self.depth_texture.view)),
            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&self.normal_texture.view)),
            builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&self.color_texture.view)),
        ];

        self.render_bundle = Some(
            builders::RenderBundleBuilder::new(ctx, "deferred")
                .with_pipeline(&self.render_pipeline)
                .with_uniform_bind_group(&self.uniform_bind_group_layout, &self.uniform_buffer)
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
