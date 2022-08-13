use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Component, Clone)]
pub struct Model {
    pub key: String,
    pub animation_times: HashMap<String, f32>,
    pub highlight: f32,
}
