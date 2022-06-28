use bevy_ecs::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct Weapon {
    pub damage: Range<f32>,
    pub time: f32,
}
