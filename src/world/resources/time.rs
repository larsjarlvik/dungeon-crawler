use std::time::Instant;

pub struct Time {
    pub last_frame: Instant,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
        }
    }
}

impl Time {
    pub fn reset(&mut self) {
        self.last_frame = Instant::now();
    }
}
