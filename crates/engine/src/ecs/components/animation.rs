use crate::config;
use bevy_ecs::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub enum AnimationSpeed {
    Original,
    Length(f32),
    Speed(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnimationRunType {
    Default,
    Repeat,
    Stopped,
}

#[derive(Clone, Debug)]
pub struct Animation {
    pub name: String,
    pub elapsed: f32,
    pub speed: f32,
    pub run_type: AnimationRunType,
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

#[derive(Component)]
pub struct Animations {
    pub channels: HashMap<String, Channel>,
}

impl Animations {
    pub fn new(key: &str, animation: &str, run_type: AnimationRunType) -> Self {
        let mut channels = HashMap::new();

        channels.insert(
            key.to_string(),
            Channel {
                prev: None,
                current: Animation {
                    name: animation.to_string(),
                    elapsed: 0.0,
                    speed: 1.0,
                    run_type,
                },
                updated: Instant::now(),
            },
        );

        Self { channels }
    }

    pub fn set_animation(&mut self, channel: &str, animation: &str, speed: AnimationSpeed, run: AnimationRunType) {
        let speed = match speed {
            AnimationSpeed::Original => 1.0,
            AnimationSpeed::Length(length) => length,
            AnimationSpeed::Speed(speed) => speed,
        };

        if let Some(channel) = self.channels.get_mut(&channel.to_string()) {
            if channel.current.name == animation.to_string() && channel.current.run_type != AnimationRunType::Stopped {
                channel.current.speed = speed;
                return;
            }

            let current_elapsed = channel.updated.elapsed().as_secs_f32();

            channel.prev = Some(channel.current.clone());
            channel.current = Animation {
                name: animation.to_string(),
                speed,
                elapsed: 0.0,
                run_type: run,
            };
            channel.updated = Instant::now() - Duration::from_secs_f32((config::ANIMATION_BLEND_SECONDS - current_elapsed).max(0.0));
        } else {
            self.channels.insert(
                channel.to_string(),
                Channel {
                    prev: None,
                    current: Animation {
                        name: animation.to_string(),
                        speed,
                        elapsed: 0.0,
                        run_type: run,
                    },
                    updated: Instant::now(),
                },
            );
        }
    }
}
