#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub background: [f32; 4],
    pub foreground: [f32; 4],
    pub viewport_size: [f32; 2],
    pub variant: u32,
    pub opacity: f32,
    pub has_image: u32,
}
