use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self {
            max: amount,
            current: amount,
        }
    }
}
