use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
