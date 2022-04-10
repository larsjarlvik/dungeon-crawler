use bevy_ecs::prelude::*;

use crate::engine;

#[derive(Component)]
pub struct Model {
    pub model: engine::ModelMetaData,
    pub highlight: f32,
}

impl Model {
    pub fn new(model: engine::ModelMetaData, highlight: f32) -> Self {
        Self { model, highlight }
    }
}
