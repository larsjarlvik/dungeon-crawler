use bevy_ecs::prelude::*;
use rand::Rng;

#[derive(Component, Debug)]
pub struct Flicker {
    pub amount: f32,
    pub last: f32,
    pub speed: f32,
}

impl Flicker {
    pub fn new(amount: f32, speed: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            amount,
            last: rng.gen::<f32>(),
            speed,
        }
    }
}
