use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Health {
    pub amount: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}
