use cgmath::*;
use specs::{Component, VecStorage};

pub struct Movement {
    pub max_velocity: f32,
    pub velocity: Vector3<f32>,
}

impl Component for Movement {
    type Storage = VecStorage<Self>;
}

impl Movement {
    pub fn new(max_velocity: f32) -> Self {
        Self {
            max_velocity,
            velocity: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn towards(&mut self, direction: Vector3<f32>) {
        self.velocity += direction;
        self.velocity.x = self.velocity.x.min(self.max_velocity).max(-self.max_velocity);
        self.velocity.y = self.velocity.y.min(self.max_velocity).max(-self.max_velocity);
        self.velocity.z = self.velocity.z.min(self.max_velocity).max(-self.max_velocity);
    }
}
