use bevy_ecs::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct Attack {
    pub team: usize,
    pub damage: Range<f32>,
}
