use bevy_ecs::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct Attack {
    pub collision_key: String,
    pub damage: Range<f32>,
}
