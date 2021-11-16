#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub start_color: [f32; 4],
    pub end_color: [f32; 4],
    pub life: [f32; 4],
}
