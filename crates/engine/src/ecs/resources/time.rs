use bevy_ecs::system::Resource;
use std::time::Instant;

#[derive(Resource)]
pub struct Time {
    pub total_time: Instant,
    pub time: Instant,
    pub last_frame: f32,
    pub accumulator: f32,
    pub alpha: f32,
    pub frame: u32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            total_time: Instant::now(),
            time: Instant::now(),
            last_frame: 0.0,
            accumulator: 0.0,
            alpha: 0.0,
            frame: 0,
        }
    }
}

impl Time {
    pub fn freeze(&mut self, accumulator: f32, time_step: f32) {
        self.last_frame = self.time.elapsed().as_secs_f32();
        self.alpha = accumulator / time_step;
        self.accumulator = accumulator;
        self.time = Instant::now();
        self.frame += 1;
    }
}
