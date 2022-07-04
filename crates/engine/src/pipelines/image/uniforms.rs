#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub viewport_size: [f32; 2],
}
