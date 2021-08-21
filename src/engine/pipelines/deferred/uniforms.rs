#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub lights: [LightUniforms; 10],
    pub lights_count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniforms {
    pub position: [f32; 4],
    pub direction: [f32; 4],
    pub color: [f32; 4],
}

impl Default for LightUniforms {
    fn default() -> LightUniforms {
        LightUniforms {
            position: [0.0, 0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}
