use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Component)]
pub struct Movement {
    pub max_velocity: f32,
    pub velocity: f32,
    pub direction: f32,
}

impl Movement {
    pub fn new(max_velocity: f32) -> Self {
        Self {
            max_velocity,
            velocity: 0.0,
            direction: 0.0,
        }
    }

    pub fn towards(&mut self, direction: Vector3<f32>) {
        self.direction = direction.x.atan2(direction.z);
        self.velocity = self.velocity.min(self.max_velocity).max(-self.max_velocity);
    }
}
