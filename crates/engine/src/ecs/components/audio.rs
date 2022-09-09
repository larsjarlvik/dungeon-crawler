use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Sound {
    pub name: String,
    pub played: bool,
}

impl Sound {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            played: false,
        }
    }
}
