use crate::{pipelines::builders, texture, Engine};
use cgmath::*;
use fxhash::FxHashMap;
use wgpu::util::DeviceExt;

use super::uniforms::{Uniforms, UniformsTextured};

#[derive(Debug)]
pub enum AssetState {
    Default,
    Hover,
    Pressed,
}

#[derive(Debug)]
pub struct Data {
    pub position: Point2<f32>,
    pub size: Point2<f32>,
    pub background: Vector4<f32>,
    pub background_end: Vector4<f32>,
    pub gradient_angle: f32,
    pub foreground: Vector4<f32>,
    pub opacity: f32,
    pub border_radius: f32,
    pub shadow_radius: f32,
    pub shadow_color: Vector4<f32>,
    pub shadow_offset: Vector2<f32>,
    pub clip: Option<[u32; 4]>,
}

pub struct RenderBundle {
    pub uniform_bind_group: wgpu::BindGroup,
    pub clip: Option<[u32; 4]>,
}

#[derive(Default)]
pub struct ImageContext {
    pub textures: FxHashMap<String, wgpu::BindGroup>,
    pub queue: Vec<(Option<String>, RenderBundle)>,
}

impl ImageContext {
    pub fn add_texture(engine: &mut Engine, key: &str, data: Vec<u8>) {
        let img = image::load_from_memory(data.as_slice()).unwrap();
        let pixels = img.as_bytes();

        let texture = texture::Texture::create_view(&engine.ctx, pixels, img.width(), img.height(), true);
        let sampler = texture::Texture::create_sampler(&engine.ctx, wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear, None);

        engine.ctx.images.textures.insert(
            key.to_string(),
            engine.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("asset_uniform_bind_group"),
                layout: &engine.ui_pipeline.texture_bind_group_layout.layout,
                entries: &[
                    builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&texture.view)),
                    builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&sampler)),
                ],
            }),
        );
    }

    pub fn create_item(engine: &mut Engine, data: Data, texture_id: Option<String>) -> RenderBundle {
        let builder = builders::RenderBundleBuilder::new(&engine.ctx, "asset");

        let uniform_bind_group = if texture_id.is_some() {
            let uniform_buffer = engine.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("image_context_texture_buffer"),
                contents: bytemuck::cast_slice(&[UniformsTextured {
                    position: data.position.into(),
                    size: data.size.into(),
                    viewport_size: [engine.ctx.viewport.width as f32, engine.ctx.viewport.height as f32],
                    foreground: data.foreground.into(),
                    opacity: data.opacity,
                    pad: [0.0; 4],
                }]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            builder.create_uniform_bind_group(&engine.ui_pipeline.uniform_bind_group_layout, &uniform_buffer)
        } else {
            let uniform_buffer = engine.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("image_context_texture_buffer"),
                contents: bytemuck::cast_slice(&[Uniforms {
                    position: data.position.into(),
                    size: data.size.into(),
                    viewport_size: [engine.ctx.viewport.width as f32, engine.ctx.viewport.height as f32],
                    background: data.background.into(),
                    background_end: data.background_end.into(),
                    foreground: data.foreground.into(),
                    opacity: data.opacity,
                    border_radius: data.border_radius,
                    shadow_radius: data.shadow_radius,
                    shadow_color: data.shadow_color.into(),
                    shadow_offset: data.shadow_offset.into(),
                    gradient_angle: data.gradient_angle.to_radians(),
                    pad: [0.0; 12],
                }]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            builder.create_uniform_bind_group(&engine.ui_pipeline.uniform_bind_group_layout, &uniform_buffer)
        };

        RenderBundle {
            uniform_bind_group,
            clip: data.clip,
        }
    }

    pub fn queue(&mut self, bundle: RenderBundle, texture_id: Option<String>) {
        self.queue.push((texture_id, bundle));
    }
}
