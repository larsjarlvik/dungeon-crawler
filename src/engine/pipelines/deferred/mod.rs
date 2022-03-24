use crate::{
    config,
    engine::{self, pipelines::builders, texture},
    utils::Interpolate,
    world::{components, resources},
};
use bevy_ecs::prelude::World;
use cgmath::*;
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
    pub base_shadow_size: f32,
}

impl DeferredPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let pipeline_builder = builders::PipelineBuilder::new(ctx, "deferred");

        let (width, height) = ctx.viewport.get_render_size();
        let depth_texture = texture::Texture::create_depth_texture(&ctx, width, height, "deferred_depth_texture");
        let normal_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_normal_texture");
        let color_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_color_texture");
        let orm_texture = texture::Texture::create_texture(ctx, config::COLOR_TEXTURE_FORMAT, width, height, "deferred_orm_texture");
        let base_shadow_size = ctx.device.limits().max_texture_dimension_2d as f32 / 4.0;

        let shadow_texture = texture::Texture::create_depth_texture(
            &ctx,
            (base_shadow_size * ctx.settings.shadow_map_scale) as u32,
            (base_shadow_size * ctx.settings.shadow_map_scale) as u32,
            "deferred_shadow_texture",
        );

        let shadow_sampler = texture::Texture::create_sampler(
            ctx,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            Some(wgpu::CompareFunction::LessEqual),
        );

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
            base_shadow_size,
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
            (self.base_shadow_size * ctx.settings.shadow_map_scale) as u32,
            (self.base_shadow_size * ctx.settings.shadow_map_scale) as u32,
            "deferred_shadow_texture",
        );
    }

    pub fn update(&mut self, ctx: &engine::Context, components: &mut World) {
        let (view_proj, shadow_matrix, eye) = {
            let camera = components.get_resource::<resources::Camera>().unwrap();
            (camera.view_proj, camera.get_shadow_matrix(), camera.eye)
        };

        let (lights_count, lights) = self.get_lights(ctx, components);

        let uniforms = uniforms::Uniforms {
            inv_view_proj: view_proj.invert().unwrap().into(),
            shadow_matrix: shadow_matrix.into(),
            eye_pos: eye.to_vec().extend(0.0).into(),
            viewport_size: [ctx.viewport.get_render_width(), ctx.viewport.get_render_height(), 0.0, 0.0],
            lights,
            lights_count,
            contrast: ctx.settings.contrast,
            brightness: ctx.settings.brightness,
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

    fn get_lights(&self, ctx: &engine::Context, components: &mut World) -> (i32, [uniforms::LightUniforms; 32]) {
        let alpha = {
            let time = components.get_resource::<resources::Time>().unwrap();
            time.alpha
        };
        let (frustum, target) = {
            let camera = components.get_resource::<resources::Camera>().unwrap();
            (camera.frustum, camera.target)
        };

        let mut lights: [uniforms::LightUniforms; 32] = Default::default();

        let mut visible_lights: Vec<(&components::Light, &components::Transform)> = components
            .query::<(&components::Light, &components::Transform)>()
            .iter(&components)
            .filter(|(light, transform)| {
                if let Some(bounding_box) = &light.bounding_box {
                    frustum.test_bounding_box(&bounding_box.transform(transform.to_matrix(alpha).into()))
                } else {
                    true
                }
            })
            .collect();

        visible_lights.sort_by(|a, b| {
            a.1.translation
                .get(alpha)
                .distance(target)
                .partial_cmp(&b.1.translation.get(alpha).distance(target))
                .unwrap()
        });

        for (i, (light, transform)) in visible_lights.iter().enumerate() {
            let radius = if let Some(radius) = light.radius { radius } else { 0.0 };
            if i >= lights.len() {
                break;
            }

            lights[i] = uniforms::LightUniforms {
                position: (transform.translation.get(alpha) + light.offset.get(alpha)).into(),
                radius,
                color: (light.color * light.intensity.get(alpha)).into(),
                bloom: light.bloom * ctx.settings.bloom,
            };
        }

        (visible_lights.len() as i32, lights)
    }
}
