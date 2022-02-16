use std::time::Duration;

#[cfg(not(target_os = "android"))]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
#[cfg(target_os = "android")]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const CLEAR_COLOR: wgpu::Color = wgpu::Color::BLACK;
pub const MAX_JOINT_COUNT: usize = 64;
pub const UPDATES_PER_SECOND: f32 = 50.0;
pub const ANIMATION_BLEND_SECONDS: f32 = 0.3;
pub const JOYSTICK_RADIUS: f32 = 0.12;
pub const CAMERA_ROTATION: f32 = 45.0;
pub const Z_FAR: f32 = 25.0;

pub const GRID_COUNT: i32 = 10;
pub const GRID_DIST: f32 = 0.635;

pub const UI_TRANSITION_TIME: f32 = 0.25;
pub const JOYSTICK_SENSITIVITY: f32 = 4.0;

pub fn time_step() -> Duration {
    Duration::from_secs_f32(1.0 / UPDATES_PER_SECOND)
}
