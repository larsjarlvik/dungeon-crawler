use crate::{texture, Context};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
    pub position: [f32; 2],
    pub size: [f32; 2],
}

pub struct ImageContext {
    pub textures: HashMap<String, texture::Texture>,
    pub queue: Vec<(String, Data)>,
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

    pub fn queue(&mut self, id: String, data: Data) {
        self.queue.push((id, data));
    }
}
