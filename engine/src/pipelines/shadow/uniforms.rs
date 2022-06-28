#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub shadow_matrix: [[f32; 4]; 4],
    pub inv_view_proj: [[f32; 4]; 4],
    pub viewport_size: [f32; 4],
}
