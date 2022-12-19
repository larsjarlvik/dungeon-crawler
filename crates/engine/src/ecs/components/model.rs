use crate::{model::animation::SoundEffect, ModelInstance};
use bevy_ecs::prelude::*;
use fxhash::FxHashMap;

#[derive(Component, Clone)]
pub struct Model {
    pub key: String,
    pub animation_times: FxHashMap<String, f32>,
    pub animation_sound_effects: FxHashMap<String, SoundEffect>,
}

impl Model {
    pub fn get_model<'a>(&'a self, ctx: &'a crate::Context) -> &'a ModelInstance {
        ctx.model_instances
            .get(&self.key)
            .unwrap_or_else(|| panic!("Could not find model \"{}\"!", self.key))
    }
}
