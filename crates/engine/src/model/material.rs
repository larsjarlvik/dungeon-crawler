use crate::{texture, Context};
use cgmath::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Textures {
    pub base_color: texture::Texture,
    pub normal: texture::Texture,
    pub orm: texture::Texture,
}

pub struct Material {
    pub roughness_factor: f32,
    pub metallic_factor: f32,
    pub base_color_factor: Vector4<f32>,
    pub emissive_factor: Vector3<f32>,
    pub textures: Option<Textures>,
}

impl Material {
    pub fn new(ctx: &Context, material: &gltf::Material, images: &[gltf::image::Data]) -> Self {
        let pbr = material.pbr_metallic_roughness();

        let textures = if pbr.base_color_texture().is_some() {
            let sources = vec![
                pbr.base_color_texture().expect("Missing base_color_texture!").texture(),
                material.normal_texture().expect("Missing normal_texture!").texture(),
                pbr.metallic_roughness_texture()
                    .expect("Missing metallic_roughness_texture!")
                    .texture(),
            ];

            let mut textures: Vec<texture::Texture> = sources.into_par_iter().map(|t| load_image(ctx, t, images)).collect();
            Some(Textures {
                orm: textures.pop().unwrap(),
                normal: textures.pop().unwrap(),
                base_color: textures.pop().unwrap(),
            })
        } else {
            None
        };

        Self {
            textures,
            roughness_factor: pbr.roughness_factor(),
            metallic_factor: pbr.metallic_factor(),
            base_color_factor: Vector4::from(pbr.base_color_factor()),
            emissive_factor: Vector3::from(material.emissive_factor()),
        }
    }
}

fn load_image(ctx: &Context, texture: gltf::Texture, images: &[gltf::image::Data]) -> texture::Texture {
    let image = images.get(texture.source().index()).expect("Could not find normal texture!");
    let channels = image.pixels.len() as usize / (image.height * image.width) as usize;

    let pixels = if channels < 4 {
        let len = (image.pixels.len() / channels) * 4;
        let mut result = Vec::with_capacity(len as usize);
        let remain = vec![0; 4 - channels];

        image.pixels.chunks(channels).for_each(|c| {
            result.extend_from_slice(c);
            result.extend_from_slice(remain.as_slice());
        });

        result
    } else {
        image.pixels.clone()
    };

    texture::Texture::create_view(ctx, pixels.as_slice(), image.width, image.height, true)
}
