use bevy_ecs::prelude::Component;
use fxhash::FxHashMap;

#[derive(Clone)]
pub struct Sound {
    pub name: String,
    pub amplification: f32,
    pub started: bool,
}

impl Sound {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            amplification: 1.0,
            started: false,
        }
    }
}

#[derive(Default, Component)]
pub struct SoundEffects {
    pub sounds: FxHashMap<String, Sound>,
}

impl SoundEffects {
    pub fn set(&mut self, key: String, sound: Sound) {
        self.sounds.insert(key, sound);
    }
}
