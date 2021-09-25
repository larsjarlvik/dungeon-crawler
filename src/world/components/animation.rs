use specs::{Component, VecStorage};
use std::{collections::HashMap, time::Instant};

use crate::config;

#[derive(Clone, Debug)]
pub struct Animation {
    pub name: String,
    pub elapsed: f32,
    pub speed: f32,
}

#[derive(Debug)]
pub struct Channel {
    pub prev: Option<Animation>,
    pub current: Animation,
    pub updated: Instant,
}

impl Channel {
    pub fn get_blend_factor(&self) -> f32 {
        if self.prev.is_some() {
            return (self.updated.elapsed().as_secs_f32() / config::ANIMATION_BLEND_SECONDS).min(1.0);
        }
        1.0
    }
}

pub struct Animations {
    pub channels: HashMap<String, Channel>,
}

impl Component for Animations {
    type Storage = VecStorage<Self>;
}

impl Animations {
    pub fn new(key: &str, animation: &str) -> Self {
        let mut channels = HashMap::new();
        channels.insert(
            key.to_string(),
            Channel {
                prev: None,
                current: Animation {
                    name: animation.to_string(),
                    elapsed: 0.0,
                    speed: 1.0,
                },
                updated: Instant::now(),
            },
        );

        Self { channels }
    }

    pub fn set_animation(&mut self, channel: &str, animation: &str, speed: f32) {
        if let Some(channel) = self.channels.get_mut(&channel.to_string()) {
            if channel.current.name == animation.to_string() {
                channel.current.speed = speed;
                return;
            }

            channel.prev = Some(channel.current.clone());
            channel.current = Animation {
                name: animation.to_string(),
                speed,
                elapsed: 0.0,
            };
            channel.updated = Instant::now();
        } else {
            self.channels.insert(
                channel.to_string(),
                Channel {
                    prev: None,
                    current: Animation {
                        name: animation.to_string(),
                        speed,
                        elapsed: 0.0,
                    },
                    updated: Instant::now(),
                },
            );
        }
    }
}
