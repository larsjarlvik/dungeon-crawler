use std::time::Instant;

pub struct Time {
    time: Instant,
    pub last_frame: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            last_frame: 0.0,
        }
    }
}

impl Time {
    pub fn reset(&mut self) {
        self.time = Instant::now();
    }

    pub fn freeze(&mut self) {
        self.last_frame = self.time.elapsed().as_secs_f32();
    }
}
