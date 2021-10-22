use cgmath::*;
use std::collections::HashMap;

use super::{material, primitive};

#[derive(Debug)]
pub struct Emitter {
    pub name: String,
    pub spread: f32,
    pub speed: f32,
    pub size: f32,
    pub life_time: f32,
    pub particle_count: u32,
    pub start_color: Vector3<f32>,
    pub end_color: Vector3<f32>,
    pub position: Vector3<f32>,
    pub flicker: Option<f32>,
}

impl Emitter {
    pub fn new(gltf_mesh: &gltf::Mesh, primitive: &primitive::Primitive, materials: &Vec<material::Material>) -> Self {
        let extras: HashMap<String, f32>;

        let position = primitive.get_center();
        let material = materials
            .get(primitive.material.expect("Emitter does not have any material!"))
            .expect("Emitter material not found!");

        return if let Some(json) = gltf_mesh.extras() {
            extras = serde_json::from_str(json.get()).unwrap();

            let flicker = if let Some(flicker) = extras.get("flicker") {
                Some(*flicker)
            } else {
                None
            };

            Self {
                name: gltf_mesh.name().expect("Missing name!").to_string(),
                spread: *extras.get("spread").expect("Missing spread!"),
                speed: *extras.get("speed").expect("Missing speed!"),
                size: *extras.get("size").expect("Missing size!"),
                life_time: *extras.get("life_time").expect("Missing life_time!"),
                particle_count: *extras.get("particle_count").expect("Missing particle_count!") as u32,
                start_color: material.base_color_factor.truncate(),
                end_color: material.emissive_factor,
                flicker,
                position,
            }
        } else {
            panic!("Missing extras for particle emitter!")
        };
    }
}
