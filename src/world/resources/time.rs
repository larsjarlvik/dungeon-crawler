use std::time::Instant;

pub struct Time {
    pub total_time: Instant,
    time: Instant,
    pub last_frame: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            total_time: Instant::now(),
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
