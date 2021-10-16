use crate::{
    config,
    engine::{self, pipelines::builders, texture},
    utils::Interpolate,
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
    pub orm_texture: texture::Texture,
    pub shadow_texture: texture::Texture,
    pub shadow_sampler: wgpu::Sampler,
}

impl DeferredPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let pipeline_builder = builders::PipelineBuilder::new(ctx, "deferred");

        let (width, height) = ctx.viewport.get_render_size();
        let depth_texture = texture::Texture::create_depth_texture(&ctx, width, height, "deferred_depth_texture");
        let normal_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_normal_texture");
        let color_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_color_texture");
        let orm_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_orm_texture");
        let shadow_texture = texture::Texture::create_depth_texture(
            &ctx,
            (width as f32 * config::SHADOW_MAP_SCALE) as u32,
            (height as f32 * config::SHADOW_MAP_SCALE) as u32,
            "deferred_shadow_texture",
        );

        let shadow_sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        let uniform_bind_group_layout = pipeline_builder.create_bindgroup_layout(
            0,
            "model_uniform_bind_group_layout",
            &[pipeline_builder.create_uniform_entry(0, wgpu::ShaderStages::FRAGMENT)],
        );

        let texture_bind_group_layout = pipeline_builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                pipeline_builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT),
                pipeline_builder.create_texture_entry(1, wgpu::ShaderStages::FRAGMENT),
                pipeline_builder.create_texture_entry(2, wgpu::ShaderStages::FRAGMENT),
                pipeline_builder.create_texture_entry(3, wgpu::ShaderStages::FRAGMENT),
                pipeline_builder.create_shadow_texture_entry(4, wgpu::ShaderStages::FRAGMENT),
                pipeline_builder.create_sampler_entry(5, wgpu::ShaderStages::FRAGMENT, true),
            ],
        );

        let render_pipeline = pipeline_builder
            .with_shader("shaders/deferred.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
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
            orm_texture,
            shadow_texture,
            shadow_sampler,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            uniform_buffer,
            render_bundle: None,
        }
    }

    pub fn resize(&mut self, ctx: &engine::Context) {
        let (width, height) = ctx.viewport.get_render_size();
        self.depth_texture = texture::Texture::create_depth_texture(&ctx, width, height, "deferred_depth_texture");
        self.normal_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_normal_texture");
        self.color_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_color_texture");
        self.orm_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "orm_texture");
        self.shadow_texture = texture::Texture::create_depth_texture(
            &ctx,
            (width as f32 * config::SHADOW_MAP_SCALE) as u32,
            (height as f32 * config::SHADOW_MAP_SCALE) as u32,
            "deferred_shadow_texture",
        );
    }

    pub fn update(&mut self, ctx: &engine::Context, components: &specs::World) {
        let camera = components.read_resource::<resources::Camera>();
        let (lights_count, lights) = self.get_lights(&camera, components);

        let uniforms = uniforms::Uniforms {
            inv_view_proj: camera.view_proj.invert().unwrap().into(),
            shadow_matrix: camera.get_shadow_matrix().into(),
            eye_pos: camera.get_eye().to_vec().extend(0.0).into(),
            viewport_size: [ctx.viewport.get_render_width(), ctx.viewport.get_render_height(), 0.0, 0.0],
            lights,
            lights_count,
        };

        ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let texture_entries = &[
            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&self.depth_texture.view)),
            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&self.normal_texture.view)),
            builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&self.color_texture.view)),
            builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::TextureView(&self.orm_texture.view)),
            builders::RenderBundleBuilder::create_entry(4, wgpu::BindingResource::TextureView(&self.shadow_texture.view)),
            builders::RenderBundleBuilder::create_entry(5, wgpu::BindingResource::Sampler(&self.shadow_sampler)),
        ];

        self.render_bundle = Some(
            builders::RenderBundleBuilder::new(ctx, "deferred")
                .with_pipeline(&self.render_pipeline)
                .with_uniform_bind_group(&self.uniform_bind_group_layout, &self.uniform_buffer)
                .with_primitive(
                    builders::PrimitiveBuilder::new(ctx, "deferred")
                        .with_texture_bind_group(&self.texture_bind_group_layout, texture_entries)
                        .with_length(4),
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

    fn get_lights(&self, camera: &resources::Camera, components: &specs::World) -> (i32, [uniforms::LightUniforms; 32]) {
        let time = components.read_resource::<resources::Time>();
        let light_sources = components.read_storage::<components::Light>();
        let transform = components.read_storage::<components::Transform>();

        let mut lights: [uniforms::LightUniforms; 32] = Default::default();

        let visible_lights: Vec<(&components::Light, &components::Transform)> = (&light_sources, &transform)
            .join()
            .filter(|(light, transform)| {
                if let Some(bounding_box) = &light.bounding_box {
                    camera
                        .frustum
                        .test_bounding_box(&bounding_box.transform(transform.to_matrix(time.last_frame).into()))
                } else {
                    true
                }
            })
            .collect();

        for (i, (light, transform)) in visible_lights.iter().enumerate() {
            let radius = if let Some(radius) = light.radius { radius } else { 0.0 };

            if i >= lights.len() {
                break;
            }

            lights[i] = uniforms::LightUniforms {
                position: (transform.translation.get(time.last_frame) + light.offset).into(),
                radius,
                color: (light.color * light.intensity.get(time.last_frame)).extend(0.0).into(),
            };
        }

        (visible_lights.len() as i32, lights)
    }
}
