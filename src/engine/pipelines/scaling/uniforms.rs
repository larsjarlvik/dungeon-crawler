#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub viewport_width: f32,
    pub viewport_height: f32,
}
