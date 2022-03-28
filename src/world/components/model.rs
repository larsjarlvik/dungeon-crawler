use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Model {
    pub key: String,
    pub highlight: f32,
}

impl Model {
    pub fn new(key: &str, highlight: f32) -> Self {
        Self {
            key: key.to_string(),
            highlight,
        }
    }
}
