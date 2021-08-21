use std::time;

pub struct Time {
    pub when: time::Instant,
    pub elapsed: time::Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            when: time::Instant::now(),
            elapsed: time::Duration::new(0, 0),
        }
    }
}

impl Time {
    pub fn set(&mut self) {
        self.elapsed = self.when.elapsed();
        self.when = time::Instant::now();
    }
}
