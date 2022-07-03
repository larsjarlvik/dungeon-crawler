use bevy_ecs::prelude::*;
use engine::collision::Polygon;

#[derive(Component, Clone)]
pub struct Collision {
    pub key: String,
    pub polygons: Vec<Polygon>,
}

impl Collision {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self {
            key: uuid::Uuid::new_v4().to_string(),
            polygons,
        }
    }
}
