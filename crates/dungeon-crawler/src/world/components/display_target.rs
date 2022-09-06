use bevy_ecs::prelude::Component;

#[derive(Component, Clone)]
pub struct DisplayTarget {
    pub name: String,
    pub current_health: f32,
    pub max_health: f32,
}
