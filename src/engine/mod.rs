use crate::{config, utils};

pub mod bounding_box;
pub mod collision;
pub mod frustum;
pub mod model;
pub mod pipelines;
pub mod texture;
mod viewport;

pub struct Context {
    pub instance: wgpu::Instance,
    pub viewport: viewport::Viewport,
    pub surface: Option<wgpu::Surface>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct Engine {
    pub ctx: Context,
    pub model_pipeline: pipelines::ModelPipeline,
    pub joystick_pipeline: pipelines::JoystickPipeline,
    pub deferred_pipeline: pipelines::DeferredPipeline,
    pub particle_pipeline: pipelines::ParticlePipeline,
    pub scaling_pipeline: pipelines::ScalingPipeline,
    pub glyph_pipeline: pipelines::GlyphPipeline,
}

impl Engine {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let viewport = viewport::Viewport::new(size.width, size.height, window.scale_factor() as f32);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
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
            instance,
            viewport,
            device,
            surface: Some(surface),
            queue,
        };

        let model_pipeline = pipelines::ModelPipeline::new(&ctx);
        let deferred_pipeline = pipelines::DeferredPipeline::new(&ctx);
        let particle_pipeline = pipelines::ParticlePipeline::new(&ctx);
        let scaling_pipeline = pipelines::ScalingPipeline::new(&ctx);
        let joystick_pipeline = pipelines::JoystickPipeline::new(&ctx);
        let glyph_pipeline = pipelines::GlyphPipeline::new(&ctx);

        Self {
            ctx,
            model_pipeline,
            deferred_pipeline,
            particle_pipeline,
            scaling_pipeline,
            joystick_pipeline,
            glyph_pipeline,
        }
    }

    pub fn set_viewport(&mut self, window: &winit::window::Window) {
        let size = window.inner_size();
        self.ctx.viewport = viewport::Viewport::new(size.width, size.height, window.scale_factor() as f32);
        self.scaling_pipeline.resize(&mut self.ctx);

        if self.ctx.surface.is_none() {
            self.ctx.surface = Some(unsafe { self.ctx.instance.create_surface(window) });
        }

        if let Some(surface) = &mut self.ctx.surface {
            surface.configure(
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
    }

    pub fn get_output_frame(&self) -> Option<wgpu::SurfaceTexture> {
        if let Some(surface) = &self.ctx.surface {
            return Some(surface.get_current_texture().expect("Failed to get output frame!"));
        }

        None
    }

    pub fn load_model(&self, path: &str) -> model::GltfModel {
        let bytes = utils::read_bytes(path);
        model::GltfModel::new(&self.ctx, bytes.as_slice())
    }

    pub fn get_mesh(&self, model: &model::GltfModel, name: &str) -> pipelines::model::Model {
        pipelines::model::Model::new(&self.ctx, &self.model_pipeline, model, name)
    }
}
