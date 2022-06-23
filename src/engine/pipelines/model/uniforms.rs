use crate::config;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub inv_model: [[f32; 4]; 4],
    pub joint_transforms: [[[f32; 4]; 4]; config::MAX_JOINT_COUNT],
    pub highlight: f32,
    pub is_animated: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EnvironmentUniforms {
    pub eye_pos: [f32; 4],
    pub target: [f32; 4],
    pub lights: [LightUniforms; 32],
    pub lights_count: i32,
    pub contrast: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniforms {
    pub position: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub bloom: f32,
}

impl Default for LightUniforms {
    fn default() -> LightUniforms {
        LightUniforms {
            position: [0.0, 0.0, 0.0],
            radius: 0.0,
            color: [0.0, 0.0, 0.0],
            bloom: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PrimitiveUniforms {
    pub orm_factor: [f32; 4],
    pub base_color_factor: [f32; 4],
    pub has_textures: u32,
}
