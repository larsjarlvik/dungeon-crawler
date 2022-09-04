use crate::config;
use bevy_ecs::prelude::*;
use std::{collections::HashMap, time::Instant};

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
    pub started: Instant,
    pub speed: f32,
    pub run_type: AnimationRunType,
}

#[derive(Debug)]
pub struct Channel {
    pub queue: Vec<Animation>,
}

impl Channel {
    pub fn get_blend_factor(&self, index: usize) -> f32 {
        if let Some(animation) = self.queue.get(index) {
            let mut elapsed = animation.started.elapsed().as_secs_f32();

            if let Some(next_animation) = self.queue.get(index + 1) {
                elapsed -= next_animation.started.elapsed().as_secs_f32();
            }

            return (elapsed / config::ANIMATION_BLEND_SECONDS).min(1.0);
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
                queue: vec![Animation {
                    name: animation.to_string(),
                    elapsed: 0.0,
                    started: Instant::now(),
                    speed: 1.0,
                    run_type,
                }],
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
            if let Some(last) = channel.queue.last_mut() {
                if last.name == animation {
                    last.speed = speed;
                    return;
                }
            }

            channel.queue.push(Animation {
                name: animation.to_string(),
                speed,
                elapsed: 0.0,
                started: Instant::now(),
                run_type: run,
            });
        } else {
            self.channels.insert(
                channel.to_string(),
                Channel {
                    queue: vec![Animation {
                        name: animation.to_string(),
                        speed,
                        elapsed: 0.0,
                        started: Instant::now(),
                        run_type: run,
                    }],
                },
            );
        }
    }
}
