use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::{model::animation::SoundEffect, ModelInstance};

#[derive(Component, Clone)]
pub struct Model {
    pub key: String,
    pub animation_times: HashMap<String, f32>,
    pub animation_sound_effects: HashMap<String, SoundEffect>,
    pub highlight: f32,
}

impl Model {
    pub fn get_model<'a>(&'a self, ctx: &'a crate::Context) -> &'a ModelInstance {
        ctx.model_instances
            .get(&self.key)
            .expect(format!("Could not find model \"{}\"!", self.key).as_str())
    }
}
