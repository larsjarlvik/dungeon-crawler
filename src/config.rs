#[cfg(not(target_os = "android"))]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
#[cfg(target_os = "android")]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.192,
    g: 0.204,
    b: 0.220,
    a: 1.0,
};
