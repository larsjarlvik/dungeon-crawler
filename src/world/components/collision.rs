use crate::engine::*;
use specs::{Component, VecStorage};

pub struct Collision {
    pub polygons: Vec<collision::Polygon>,
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}

impl Collision {
    pub fn new(gltf: &model::GltfModel, name: &str) -> Option<Self> {
        if let Some(col) = gltf.collisions.get(name) {
            Some(Self { polygons: col.clone() })
        } else {
            None
        }
    }
}
