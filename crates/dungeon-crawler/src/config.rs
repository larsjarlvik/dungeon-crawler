use std::time::Duration;

pub const UPDATES_PER_SECOND: f32 = 50.0;
pub const CAMERA_ROTATION: f32 = 45.0;
pub const GRID_COUNT: i32 = 10;
pub const GRID_DIST: f32 = 0.635;
pub const UI_TRANSITION_TIME: f32 = 0.6;

pub fn time_step() -> Duration {
    Duration::from_secs_f32(1.0 / UPDATES_PER_SECOND)
}
