#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub background: [f32; 4],
    pub background_end: [f32; 4],
    pub foreground: [f32; 4],
    pub shadow_color: [f32; 4],
    pub viewport_size: [f32; 2],
    pub shadow_offset: [f32; 2],
    pub border_radius: f32,
    pub shadow_radius: f32,
    pub opacity: f32,
    pub gradient_angle: f32,
    pub pad: [f32; 12],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformsTextured {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub foreground: [f32; 4],
    pub viewport_size: [f32; 2],
    pub opacity: f32,
    pub pad: [f32; 4],
}
