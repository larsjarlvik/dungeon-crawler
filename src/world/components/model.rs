use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Model {
    pub key: String,
}

impl Model {
    pub fn new(key: &str) -> Self {
        Self { key: key.to_string() }
    }
}
