use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Attack {
    pub collision_key: String,
    pub min: f32,
    pub max: f32,
}
