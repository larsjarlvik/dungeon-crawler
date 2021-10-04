use crate::engine::{collision::Polygon, model};
use specs::{Component, VecStorage};

pub struct Collider {
    pub polygons: Vec<Polygon>,
    pub intersections: Vec<Polygon>,
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}

impl Collider {
    pub fn new(gltf: &model::GltfModel, name: &str) -> Self {
        Self {
            polygons: gltf
                .collisions
                .get(name)
                .expect(format!("Could not find collision for: {}!", name).as_str())
                .clone(),
            intersections: vec![],
        }
    }
}
