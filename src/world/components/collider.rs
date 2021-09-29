use crate::engine::collision::Polygon;
use cgmath::*;
use specs::{Component, VecStorage};

pub struct Collider {
    pub polygon: Polygon,
    pub intersections: Vec<Polygon>,
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}

impl Collider {
    pub fn new(polygon: Vec<Vector2<f32>>) -> Self {
        Self {
            polygon,
            intersections: vec![],
        }
    }
}
