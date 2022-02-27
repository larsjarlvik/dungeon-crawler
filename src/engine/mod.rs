use std::collections::HashMap;

use crate::{config, utils};

pub mod bounding_box;
pub mod collision;
pub mod frustum;
pub mod model;
pub mod pipelines;
mod settings;
pub mod texture;
mod viewport;
pub use settings::Settings;

pub struct ModelInstance {
    pub key: String,
    pub model: pipelines::model::Model,
    pub nodes: model::GltfModelNodes,
}

pub struct Context {
    pub instance: wgpu::Instance,
    pub viewport: viewport::Viewport,
    pub surface: Option<wgpu::Surface>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub settings: settings::Settings,
    pub model_instances: HashMap<String, ModelInstance>,
    pub emitter_instances: HashMap<String, pipelines::ParticleEmitter>,
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
        let settings = settings::Settings::load();

        let viewport = viewport::Viewport::new(size.width, size.height, window.scale_factor() as f32, settings.render_scale);

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
            settings,
            model_instances: HashMap::new(),
            emitter_instances: HashMap::new(),
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

    pub fn reload_pipelines(&mut self) {
        self.model_pipeline = pipelines::ModelPipeline::new(&self.ctx);
        self.deferred_pipeline = pipelines::DeferredPipeline::new(&self.ctx);
        self.particle_pipeline = pipelines::ParticlePipeline::new(&self.ctx);
        self.scaling_pipeline = pipelines::ScalingPipeline::new(&self.ctx);
        self.joystick_pipeline = pipelines::JoystickPipeline::new(&self.ctx);
        self.glyph_pipeline = pipelines::GlyphPipeline::new(&self.ctx);
    }

    pub fn set_viewport(&mut self, window: &winit::window::Window) {
        let size = window.inner_size();
        self.ctx.viewport = viewport::Viewport::new(
            size.width,
            size.height,
            window.scale_factor() as f32,
            self.ctx.settings.render_scale,
        );
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

    pub fn load_model(&mut self, path: &str) -> model::GltfModel {
        let bytes = utils::read_bytes(path);
        model::GltfModel::new(&self.ctx, bytes.as_slice())
    }

    pub fn initialize_model(&mut self, model: &model::GltfModel, name: &str, key: String) {
        self.ctx.model_instances.insert(
            key.to_string(),
            ModelInstance {
                key,
                model: pipelines::model::Model::new(&self.ctx, &self.model_pipeline, model, name),
                nodes: model.nodes.clone(),
            },
        );
    }

    pub fn initialize_particle(&mut self, emitter: pipelines::ParticleEmitter, key: String) {
        self.ctx.emitter_instances.insert(key.to_string(), emitter);
    }
}
