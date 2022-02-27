use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}
