#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub shadow_inv_vp: [[f32; 4]; 4],
    pub inv_vp_w: [f32; 4],
    pub viewport_size: [f32; 4],
    pub shadow_size: f32,
    pub _padding: [f32; 3],
}
