use cgmath::{Matrix4, SquareMatrix};
use core::time;
use specs::{Component, VecStorage};
use std::collections::HashMap;

pub struct Channel {
    pub name: String,
    pub time: time::Duration,
}

pub struct Animation {
    pub channels: HashMap<String, Channel>,
    pub joint_matrices: Vec<Matrix4<f32>>,
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

impl Animation {
    pub fn new() -> Self {
        Self {
            joint_matrices: vec![Matrix4::identity(); 20],
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
                        time: time::Duration::new(0, 0),
                    },
                );
            }
        } else {
            self.channels.remove(&key.to_string());
        }
    }
}
