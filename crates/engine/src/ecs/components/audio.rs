use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Sound {
    pub name: String,
}

impl Sound {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}
