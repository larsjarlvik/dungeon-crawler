use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Weapon {
    pub min: f32,
    pub max: f32,
    pub time: f32,
}
