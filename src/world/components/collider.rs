use crate::engine::collision::{self, Polygon};
use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct Collider {
    pub polygons: Vec<Polygon>,
    pub intersections: Vec<Polygon>,
}

impl Collider {
    pub fn new(polygons: Vec<collision::Polygon>) -> Self {
        Self {
            polygons,
            intersections: vec![],
        }
    }
}
