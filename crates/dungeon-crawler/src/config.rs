use std::time::Duration;

pub const UPDATES_PER_SECOND: f32 = 50.0;
pub const CAMERA_ROTATION: f32 = 45.0;
pub const UI_TRANSITION_TIME: f32 = 0.6;

pub const TEAM_FRIENDLY: usize = 1;
// pub const TEAM_HOSTILE: usize = 2;

pub fn time_step() -> Duration {
    Duration::from_secs_f32(1.0 / UPDATES_PER_SECOND)
}
