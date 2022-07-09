use cgmath::{Point2, Vector4};

use crate::{texture, Context};
use std::collections::HashMap;

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
    pub foreground: Vector4<f32>,
    pub opacity: f32,
}

pub struct ImageContext {
    pub textures: HashMap<String, texture::Texture>,
    pub queue: Vec<(Option<String>, Data)>,
}

impl ImageContext {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            queue: Vec::new(),
        }
    }

    pub fn add_texture(ctx: &mut Context, key: &str, data: Vec<u8>) {
        let img = image::load_from_memory(data.as_slice()).unwrap();
        let pixels = img.as_bytes();

        let texture = texture::Texture::create_view(ctx, pixels, img.width(), img.height(), true);
        ctx.images.textures.insert(key.to_string(), texture);
    }

    pub fn queue_image(&mut self, data: Data, id: Option<String>) {
        self.queue.push((id, data));
    }
}
