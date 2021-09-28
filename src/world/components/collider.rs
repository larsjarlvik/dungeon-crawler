use cgmath::*;
use specs::{Component, VecStorage};

pub struct Collider {
    pub polygon: Vec<Vector2<f32>>,
    pub is_colliding: bool,
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}

impl Collider {
    pub fn new(polygon: Vec<Vector2<f32>>) -> Self {
        Self { polygon, is_colliding: false }
    }

    pub fn transform(&self, position: Vector3<f32>) -> Self {
        let polygon = self.polygon.iter().map(|p| Vector2::new(position.x + p.x, position.z + p.y)).collect();
        Self {
            polygon,
            is_colliding: self.is_colliding,
        }
    }
}
