use crate::config;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub inv_model: [[f32; 4]; 4],
    pub joint_transforms: [[[f32; 4]; 4]; config::MAX_JOINT_COUNT],
    pub is_animated: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PrimitiveUniforms {
    pub orm_factor: [f32; 4],
    pub base_color_factor: [f32; 4],
    pub has_textures: u32,
}
