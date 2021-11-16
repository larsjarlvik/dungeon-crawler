use rand::Rng;
use specs::{Component, VecStorage};

#[derive(Debug)]
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

impl Component for Flicker {
    type Storage = VecStorage<Self>;
}
