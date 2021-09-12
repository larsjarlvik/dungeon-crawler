#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PrimitiveUniforms {
    pub orm_factor: [f32; 4],
    pub joint_transforms: [[[f32; 4]; 4]; 20],
    pub is_animated: u32,
}
