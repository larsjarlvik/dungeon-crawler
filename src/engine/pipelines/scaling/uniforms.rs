#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub viewport: [f32; 2],
    pub sharpen: u32,
    pub scale: f32,
}
