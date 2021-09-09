use crate::{config, utils};

pub mod model;
pub mod pipelines;
mod texture;
mod viewport;

pub struct Context {
    pub viewport: viewport::Viewport,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct Engine {
    pub ctx: Context,
    pub model_pipeline: pipelines::ModelPipeline,
    pub glyph_pipeline: pipelines::GlyphPipeline,
    pub deferred_pipeline: pipelines::DeferredPipeline,
}

impl Engine {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
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

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: config::COLOR_TEXTURE_FORMAT,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Immediate,
            },
        );

        let ctx = Context {
            viewport,
            device,
            surface,
            queue,
        };

        let model_pipeline = pipelines::ModelPipeline::new(&ctx);
        let glyph_pipeline = pipelines::GlyphPipeline::new(&ctx);
        let deferred_pipeline = pipelines::DeferredPipeline::new(&ctx);

        Self {
            ctx,
            model_pipeline,
            glyph_pipeline,
            deferred_pipeline,
        }
    }

    pub fn init(&mut self) {
        self.model_pipeline = pipelines::ModelPipeline::new(&self.ctx);
        self.glyph_pipeline = pipelines::GlyphPipeline::new(&self.ctx);
        self.deferred_pipeline = pipelines::DeferredPipeline::new(&self.ctx);
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, scale_factor: f64) {
        self.ctx.viewport = viewport::Viewport::new(width, height, scale_factor);

        self.ctx.surface.configure(
            &self.ctx.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: config::COLOR_TEXTURE_FORMAT,
                width: self.ctx.viewport.width,
                height: self.ctx.viewport.height,
                present_mode: wgpu::PresentMode::Immediate,
            },
        );
    }

    pub fn get_output_frame(&self) -> wgpu::SurfaceTexture {
        self.ctx.surface.get_current_frame().expect("Failed to get output frame!").output
    }

    pub fn load_model(&self, path: &str) -> model::GltfModel {
        let bytes = utils::read_bytes(path);
        model::GltfModel::new(&self.ctx, bytes.as_slice())
    }
}
