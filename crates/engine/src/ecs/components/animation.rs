use crate::config;
use bevy_ecs::prelude::*;
use fxhash::FxHashMap;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub enum AnimationSpeed {
    Original,
    Length(f32),
    Speed(f32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationStatus {
    Default,
    Repeat,
    Stopped,
}

#[derive(Clone, Debug)]
pub struct Animation {
    pub name: String,
    pub elapsed: f32,
    pub started: Instant,
    pub speed: AnimationSpeed,
    pub status: AnimationStatus,
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
    pub channels: FxHashMap<String, Channel>,
}

impl Animations {
    pub fn new(key: &str, animation: &str, status: AnimationStatus) -> Self {
        let mut channels = FxHashMap::default();

        channels.insert(
            key.to_string(),
            Channel {
                queue: vec![Animation {
                    name: animation.to_string(),
                    elapsed: 0.0,
                    started: Instant::now(),
                    speed: AnimationSpeed::Original,
                    status,
                }],
            },
        );

        Self { channels }
    }

    pub fn set_animation(&mut self, channel: &str, animation: &str, speed: AnimationSpeed, run: AnimationStatus) {
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
                status: run,
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
                        status: run,
                    }],
                },
            );
        }
    }

    pub fn set_speed(&mut self, channel: &str, animation: &str, speed: AnimationSpeed) {
        if let Some(channel) = self.channels.get_mut(&channel.to_string()) {
            for animation in channel.queue.iter_mut().filter(|a| a.name == animation) {
                animation.speed = speed.clone();
            }
        }
    }
}
