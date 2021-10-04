use crate::engine::*;
use specs::{Component, VecStorage};

pub struct Collision {
    pub polygon: collision::Polygon,
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}

impl Collision {
    pub fn new(gltf: &model::GltfModel, name: &str) -> Self {
        Self {
            polygon: gltf
                .collisions
                .get(name)
                .expect(format!("Could not find collision for: {}!", name).as_str())
                .clone(),
        }
    }
}
