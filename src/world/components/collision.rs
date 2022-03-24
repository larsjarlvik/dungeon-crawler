use crate::engine::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Collision {
    pub polygons: Vec<collision::Polygon>,
}

impl Collision {
    pub fn new(polygons: Vec<collision::Polygon>) -> Self {
        Self { polygons }
    }
}
