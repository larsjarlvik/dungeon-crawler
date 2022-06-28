use crate::ModelMetaData;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Model {
    pub model: ModelMetaData,
    pub highlight: f32,
}

impl Model {
    pub fn new(model: ModelMetaData, highlight: f32) -> Self {
        Self { model, highlight }
    }
}
