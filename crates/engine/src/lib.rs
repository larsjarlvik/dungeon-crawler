use cgmath::*;
use pipelines::ui_element::context::ImageContext;
use std::collections::HashMap;
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder};
pub mod bounding_box;
pub mod bounding_sphere;
pub mod collision;
pub mod config;
pub mod ecs;
pub mod file;
pub mod frustum;
pub mod interpolated_value;
pub mod model;
pub mod pipelines;
mod settings;
pub mod texture;
pub mod transform;
pub mod utils;
mod viewport;
pub use settings::Settings;
pub mod audio;
use smaa::{SmaaMode, SmaaTarget};

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
    pub glyph_brush: GlyphBrush<()>,
    pub images: ImageContext,
    pub audio: audio::Player,
}

pub struct Engine {
    pub ctx: Context,
    pub model_pipeline: pipelines::ModelPipeline,
    pub shadow_pipeline: pipelines::ShadowPipeline,
    pub joystick_pipeline: pipelines::JoystickPipeline,
    pub particle_pipeline: pipelines::ParticlePipeline,
    pub scaling_pipeline: pipelines::ScalingPipeline,
    pub glyph_pipeline: pipelines::GlyphPipeline,
    pub ui_pipeline: pipelines::UiElementPipeline,
    pub smaa_target: SmaaTarget,
}

impl Engine {
    pub async fn new<W: raw_window_handle::HasRawWindowHandle>(
        window: &W,
        size: Point2<u32>,
        scale_factor: f32,
        font_data: Vec<u8>,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let settings = settings::Settings::load();

        let viewport = viewport::Viewport::new(size.x, size.y, scale_factor, settings.render_scale);

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

        configure_surface(&surface, &device, size);

        let font = FontArc::try_from_vec(font_data).expect("Failed to load font!");
        let glyph_brush = GlyphBrushBuilder::using_font(font.clone()).build(&device, config::COLOR_TEXTURE_FORMAT);

        let ctx = Context {
            instance,
            viewport,
            device,
            surface: Some(surface),
            queue,
            settings,
            model_instances: HashMap::new(),
            emitter_instances: HashMap::new(),
            glyph_brush,
            images: ImageContext::new(),
            audio: audio::Player::new(),
        };

        let model_pipeline = pipelines::ModelPipeline::new(&ctx);
        let shadow_pipeline = pipelines::ShadowPipeline::new(&ctx);
        let particle_pipeline = pipelines::ParticlePipeline::new(&ctx);
        let scaling_pipeline = pipelines::ScalingPipeline::new(&ctx);
        let joystick_pipeline = pipelines::JoystickPipeline::new(&ctx);
        let glyph_pipeline = pipelines::GlyphPipeline::new();
        let image_pipeline = pipelines::UiElementPipeline::new(&ctx);
        let smaa_target = SmaaTarget::new(
            &ctx.device,
            &ctx.queue,
            size.x,
            size.y,
            config::COLOR_TEXTURE_FORMAT,
            if ctx.settings.smaa { SmaaMode::Smaa1X } else { SmaaMode::Disabled },
        );

        Self {
            ctx,
            model_pipeline,
            shadow_pipeline,
            particle_pipeline,
            scaling_pipeline,
            joystick_pipeline,
            glyph_pipeline,
            ui_pipeline: image_pipeline,
            smaa_target,
        }
    }

    pub fn reload_pipelines(&mut self) {
        self.ui_pipeline = pipelines::UiElementPipeline::new(&self.ctx);
        self.model_pipeline = pipelines::ModelPipeline::new(&self.ctx);
        self.particle_pipeline = pipelines::ParticlePipeline::new(&self.ctx);
        self.scaling_pipeline = pipelines::ScalingPipeline::new(&self.ctx);
        self.joystick_pipeline = pipelines::JoystickPipeline::new(&self.ctx);
        self.glyph_pipeline = pipelines::GlyphPipeline::new();
        self.smaa_target = SmaaTarget::new(
            &self.ctx.device,
            &self.ctx.queue,
            self.ctx.viewport.width,
            self.ctx.viewport.height,
            config::COLOR_TEXTURE_FORMAT,
            if self.ctx.settings.smaa {
                SmaaMode::Smaa1X
            } else {
                SmaaMode::Disabled
            },
        );
    }

    pub fn set_viewport<W: raw_window_handle::HasRawWindowHandle>(&mut self, window: &W, size: Point2<u32>, scale_factor: f32) {
        self.ctx.viewport = viewport::Viewport::new(size.x, size.y, scale_factor, self.ctx.settings.render_scale);
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
                    present_mode: wgpu::PresentMode::AutoNoVsync,
                },
            );
        }

        self.smaa_target.resize(&self.ctx.device, size.x, size.y);
    }

    pub fn get_output_frame(&self) -> Option<(wgpu::SurfaceTexture, wgpu::TextureView)> {
        if let Some(surface) = &self.ctx.surface {
            let frame = surface.get_current_texture().expect("Failed to get output frame!");
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            return Some((frame, view));
        }

        None
    }

    pub fn initialize_model(&mut self, gltf_model: &model::GltfModel, name: &str, highlight: f32) -> ecs::components::Model {
        let model = pipelines::model::Model::new(&self.ctx, &self.model_pipeline, gltf_model, name);
        let nodes = gltf_model.nodes.clone();
        let animation_times = nodes.animations.iter().map(|(a, b)| (a.clone(), b.total_time)).collect();
        let key = uuid::Uuid::new_v4().to_string();

        self.ctx.model_instances.insert(
            key.to_string(),
            ModelInstance {
                key: key.clone(),
                model,
                nodes,
            },
        );

        ecs::components::Model {
            key: key.to_string(),
            animation_times,
            highlight,
        }
    }

    pub fn initialize_particle(&mut self, emitter: pipelines::ParticleEmitter, key: String) {
        self.ctx.emitter_instances.insert(key.to_string(), emitter);
    }
}

pub fn configure_surface(surface: &wgpu::Surface, device: &wgpu::Device, size: Point2<u32>) {
    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: config::COLOR_TEXTURE_FORMAT,
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::AutoNoVsync,
        },
    );
}

pub fn load_model(ctx: &Context, path: &str) -> model::GltfModel {
    let bytes = file::read_bytes(path);
    model::GltfModel::new(ctx, bytes.as_slice())
}
