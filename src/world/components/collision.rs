use cgmath::*;
use specs::{Component, VecStorage};

pub struct Collision {
    pub polygon: Vec<Vector2<f32>>,
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}

impl Collision {
    pub fn new(polygon: Vec<Vector2<f32>>) -> Self {
        Self { polygon }
    }

    pub fn transform(&self, position: Vector3<f32>) -> Self {
        let polygon = self.polygon.iter().map(|p| Vector2::new(position.x + p.x, position.z + p.y)).collect();
        Self {
            polygon
        }
    }
}
