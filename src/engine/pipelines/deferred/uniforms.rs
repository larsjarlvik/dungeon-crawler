#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub inv_view_proj: [[f32; 4]; 4],
    pub eye_pos: [f32; 4],
    pub viewport_size: [f32; 4],
    pub lights: [LightUniforms; 10],
    pub lights_count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniforms {
    pub position: [f32; 3],
    pub attenuation: f32,
    pub direction: [f32; 3],
    pub directional: u32,
    pub color: [f32; 4],
}

impl Default for LightUniforms {
    fn default() -> LightUniforms {
        LightUniforms {
            position: [0.0, 0.0, 0.0],
            attenuation: 0.0,
            direction: [0.0, 0.0, 0.0],
            directional: 0,
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}
