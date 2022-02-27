use crate::engine::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Collision {
    pub polygons: Vec<collision::Polygon>,
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
