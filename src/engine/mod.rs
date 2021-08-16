use crate::config;

pub mod model;
pub mod pipelines;
mod texture;
mod viewport;

pub struct Context {
    pub viewport: viewport::Viewport,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
    pub depth_texture: texture::Texture,
}

pub struct Engine {
    pub ctx: Context,
    pub model_pipeline: pipelines::ModelPipeline,
    pub glyph_pipeline: pipelines::GlyphPipeline,
}

impl Engine {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let viewport = viewport::Viewport::new(size.width, size.height, window.scale_factor());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No suitable GPU adapters found on the system!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: config::COLOR_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let depth_texture = texture::Texture::create_depth_texture(&device, viewport.width, viewport.height, "engine_depth_texture");

        let ctx = Context {
            viewport,
            device,
            surface,
            queue,
            swap_chain,
            depth_texture,
        };

        let model_pipeline = pipelines::ModelPipeline::new(&ctx);
        let glyph_pipeline = pipelines::GlyphPipeline::new(&ctx);

        Self {
            ctx,
            model_pipeline,
            glyph_pipeline,
        }
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, scale_factor: f64) {
        self.ctx.viewport = viewport::Viewport::new(width, height, scale_factor);
        self.ctx.swap_chain = self.ctx.device.create_swap_chain(
            &self.ctx.surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                format: config::COLOR_TEXTURE_FORMAT,
                width: self.ctx.viewport.width,
                height: self.ctx.viewport.height,
                present_mode: wgpu::PresentMode::Immediate,
            },
        );
    }

    pub fn set_depth_texture(&mut self) {
        self.ctx.depth_texture = texture::Texture::create_depth_texture(
            &self.ctx.device,
            self.ctx.viewport.width,
            self.ctx.viewport.height,
            "engine_depth_texture",
        );
    }

    pub fn get_output_frame(&self) -> wgpu::SwapChainTexture {
        self.ctx.swap_chain.get_current_frame().unwrap().output
    }

    pub fn load_model(&self, bytes: &'static [u8]) -> model::GltfModel {
        model::GltfModel::new(&self.ctx, bytes)
    }
}
