use crate::engine::{self, texture};

pub struct Material {
    pub base_color_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub orm_texture: texture::Texture,
    pub roughness_factor: f32,
    pub metllic_factor: f32,
}

impl Material {
    pub fn new(ctx: &engine::Context, material: &gltf::Material, images: &Vec<gltf::image::Data>) -> Self {
        let pbr = material.pbr_metallic_roughness();

        let base_color_texture = load_image(
            ctx,
            pbr.base_color_texture().expect("Missing base_color_texture!").texture(),
            images,
        );
        let normal_texture = load_image(
            ctx,
            material.normal_texture().expect("Missing base_color_texture!").texture(),
            images,
        );
        let occlusion_roughness_metallic_texture = load_image(
            ctx,
            pbr.metallic_roughness_texture().expect("Missing base_color_texture!").texture(),
            images,
        );

        Self {
            base_color_texture,
            normal_texture,
            orm_texture: occlusion_roughness_metallic_texture,
            roughness_factor: pbr.roughness_factor(),
            metllic_factor: pbr.metallic_factor(),
        }
    }
}

fn load_image(ctx: &engine::Context, texture: gltf::Texture, images: &Vec<gltf::image::Data>) -> texture::Texture {
    let image = images.iter().nth(texture.source().index()).expect("Could not find normal texture!");

    let mut pixels = vec![];
    if image.format == gltf::image::Format::R8G8B8 {
        for chunk in image.pixels.chunks(3) {
            pixels.append(&mut chunk.to_vec());
            pixels.push(255);
        }
    } else {
        pixels = image.pixels.clone();
    }

    texture::Texture::create_mipmapped_view(&ctx, pixels.as_slice(), image.width, image.height)
}
