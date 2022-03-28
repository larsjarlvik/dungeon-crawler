use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct Agressor {
    pub start_range: f32,
    pub end_range: f32,
    pub is_aggressive: bool,
}

impl Agressor {
    pub fn new(range: f32) -> Self {
        Self {
            start_range: range,
            end_range: range * 1.5,
            is_aggressive: false,
        }
    }
}

#[derive(Component)]
pub struct Target;
