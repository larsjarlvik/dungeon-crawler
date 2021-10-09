use crate::engine::{self, texture};
use specs::{prelude::ParallelIterator, rayon::iter::IntoParallelIterator};

pub struct Material {
    pub base_color_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub orm_texture: texture::Texture,
    pub roughness_factor: f32,
    pub metallic_factor: f32,
}

impl Material {
    pub fn new(ctx: &engine::Context, material: &gltf::Material, images: &Vec<gltf::image::Data>) -> Self {
        let pbr = material.pbr_metallic_roughness();
        let sources = vec![
            pbr.base_color_texture().expect("Missing base_color_texture!").texture(),
            material.normal_texture().expect("Missing normal_texture!").texture(),
            pbr.metallic_roughness_texture()
                .expect("Missing metallic_roughness_texture!")
                .texture(),
        ];

        let mut textures: Vec<texture::Texture> = sources.into_par_iter().map(|t| load_image(ctx, t, images)).collect();
        let orm_texture = textures.pop().unwrap();
        let normal_texture = textures.pop().unwrap();
        let base_color_texture = textures.pop().unwrap();

        Self {
            base_color_texture,
            normal_texture,
            orm_texture,
            roughness_factor: pbr.roughness_factor(),
            metallic_factor: pbr.metallic_factor(),
        }
    }
}

fn load_image(ctx: &engine::Context, texture: gltf::Texture, images: &Vec<gltf::image::Data>) -> texture::Texture {
    let image = images.iter().nth(texture.source().index()).expect("Could not find normal texture!");
    let channels = image.pixels.len() as u32 / (image.height * image.width);

    let pixels = if channels < 4 {
        let mut pixels = Vec::with_capacity(image.pixels.len() / channels as usize * 4);
        for chunk in image.pixels.chunks(channels as usize) {
            pixels.append(&mut chunk.to_vec());
            pixels.append(&mut vec![255; 4 - channels as usize]);
        }
        pixels
    } else {
        image.pixels.clone()
    };

    texture::Texture::create_mipmapped_view(&ctx, pixels.as_slice(), image.width, image.height)
}
