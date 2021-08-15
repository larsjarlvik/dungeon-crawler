use specs::{Component, VecStorage};
use std::time::{self, Instant};

pub struct Fps {
    pub last_update: time::Instant,
    pub fps: u32,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            fps: 0,
        }
    }
}

impl Component for Fps {
    type Storage = VecStorage<Self>;
}
