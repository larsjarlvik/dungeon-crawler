use crate::{pipelines::builders, texture, Engine};
use cgmath::*;
use fxhash::FxHashMap;

use super::uniforms::Uniforms;

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

pub struct QueuedElement {
    pub uniforms: Uniforms,
    pub clip: Option<[u32; 4]>,
    pub texture_id: Option<String>,
}

#[derive(Default)]
pub struct ImageContext {
    pub textures: FxHashMap<String, wgpu::BindGroup>,
    pub queue: Vec<QueuedElement>,
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

    pub fn queue_item(&mut self, data: Data, viewport: [f32; 2], texture_id: Option<String>) {
        self.queue.push(QueuedElement {
            uniforms: Uniforms {
                position: data.position.into(),
                size: data.size.into(),
                viewport_size: viewport,
                background: data.background.into(),
                background_end: data.background_end.into(),
                foreground: data.foreground.into(),
                opacity: data.opacity,
                border_radius: data.border_radius,
                shadow_radius: data.shadow_radius,
                shadow_color: data.shadow_color.into(),
                shadow_offset: data.shadow_offset.into(),
                gradient_angle: data.gradient_angle.to_radians(),
                _pad: [0.0; 36],
            },
            clip: data.clip,
            texture_id,
        });
    }
}
