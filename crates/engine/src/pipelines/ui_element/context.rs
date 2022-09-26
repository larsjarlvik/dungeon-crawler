use crate::{texture, Context};
use cgmath::*;
use fxhash::FxHashMap;

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
}

#[derive(Default)]
pub struct ImageContext {
    pub textures: FxHashMap<String, texture::Texture>,
    pub queue: Vec<(Option<String>, Data)>,
}

impl ImageContext {
    pub fn add_texture(ctx: &mut Context, key: &str, data: Vec<u8>) {
        let img = image::load_from_memory(data.as_slice()).unwrap();
        let pixels = img.as_bytes();

        let texture = texture::Texture::create_view(ctx, pixels, img.width(), img.height(), true);
        ctx.images.textures.insert(key.to_string(), texture);
    }

    pub fn queue(&mut self, data: Data, id: Option<String>) {
        self.queue.push((id, data));
    }
}
