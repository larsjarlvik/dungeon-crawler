use crate::config;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub ssao_kernel: [[f32; 4]; config::SSAO_KERNEL_SIZE],
    pub viewport: [f32; 4],
}
