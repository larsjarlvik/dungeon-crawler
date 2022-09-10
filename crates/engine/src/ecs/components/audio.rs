use std::collections::HashMap;

use bevy_ecs::prelude::Component;

#[derive(Clone)]
pub struct Sound {
    pub name: String,
    pub amplification: f32,
}

impl Sound {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            amplification: 1.0,
        }
    }
}

#[derive(Component)]
pub struct SoundEffects {
    pub sounds: HashMap<String, Sound>,
}

impl SoundEffects {
    pub fn new() -> Self {
        Self { sounds: HashMap::new() }
    }

    pub fn set(&mut self, key: String, sound: Sound) {
        self.sounds.insert(key, sound);
    }
}
