use std::time::{self, Instant};

pub struct Fps {
    last_update: time::Instant,
    fps_counter: u32,
    pub fps: u32,
}

impl Default for Fps {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            fps_counter: 0,
            fps: 0,
        }
    }
}

impl Fps {
    pub fn update(&mut self) {
        self.fps_counter += 1;

        if self.last_update.elapsed().as_millis() >= 1000 {
            self.fps = self.fps_counter;
            self.fps_counter = 0;
            self.last_update = Instant::now();
        }
    }
}
