use crate::engine::collision::{self, Polygon};
use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct Collision {
    pub key: String,
    pub polygons: Vec<Polygon>,
}

impl Collision {
    pub fn new(polygons: Vec<collision::Polygon>) -> Self {
        Self {
            key: uuid::Uuid::new_v4().to_string(),
            polygons,
        }
    }
}
