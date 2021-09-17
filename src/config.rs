use std::time::Duration;

#[cfg(not(target_os = "android"))]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
#[cfg(target_os = "android")]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const CLEAR_COLOR: wgpu::Color = wgpu::Color::BLACK;
pub const MAX_JOINT_COUNT: usize = 20;
pub const UPDATES_PER_SECOND: f32 = 50.0;

pub fn time_step() -> Duration {
    Duration::from_secs_f32(1.0 / UPDATES_PER_SECOND)
}
