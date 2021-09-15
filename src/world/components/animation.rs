use specs::{Component, VecStorage};
use std::{collections::HashMap, time::Instant};

pub struct Channel {
    pub name: String,
    pub start: Instant,
}

pub struct Animation {
    pub channels: HashMap<String, Channel>,
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

impl Animation {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn set_animation(&mut self, key: &str, animation: &str, enabled: bool) {
        if enabled {
            if !self.channels.contains_key(key) {
                self.channels.insert(
                    key.to_string(),
                    Channel {
                        name: animation.to_string(),
                        start: Instant::now(),
                    },
                );
            }
        } else {
            self.channels.remove(&key.to_string());
        }
    }
}
