use cgmath::*;
use fxhash::FxHashMap;
use pipelines::ui_element::context::ImageContext;
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
    pub model_instances: FxHashMap<String, ModelInstance>,
    pub emitter_instances: FxHashMap<String, pipelines::ParticleEmitter>,
    pub images: ImageContext,
    pub color_format: wgpu::TextureFormat,
    font_data: Vec<u8>,
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
    pub async fn new<W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle>(
        window: &W,
        size: Point2<u32>,
        scale_factor: f32,
        font_data: Vec<u8>,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });
        let surface = unsafe { instance.create_surface(window).expect("Failed to create surface!") };
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

        let preferred_formats = [
            wgpu::TextureFormat::Rgb10a2Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Bgra8Unorm,
        ];

        let caps = surface.get_capabilities(&adapter);
        let mut supported_formats = caps.formats;
        supported_formats.sort_by(|a, b| {
            preferred_formats
                .iter()
                .position(|&f| f == *a)
                .unwrap_or(1000)
                .cmp(&preferred_formats.iter().position(|f| f == b).unwrap_or(1000))
        });

        let color_format = *supported_formats.first().expect("Failed to select color format!");

        configure_surface(&surface, &device, color_format, size);

        let ctx = Context {
            instance,
            viewport,
            device,
            surface: Some(surface),
            queue,
            settings,
            model_instances: FxHashMap::default(),
            emitter_instances: FxHashMap::default(),
            images: ImageContext::default(),
            font_data,
            color_format,
        };

        let model_pipeline = pipelines::ModelPipeline::new(&ctx);
        let shadow_pipeline = pipelines::ShadowPipeline::new(&ctx);
        let particle_pipeline = pipelines::ParticlePipeline::new(&ctx);
        let scaling_pipeline = pipelines::ScalingPipeline::new(&ctx);
        let joystick_pipeline = pipelines::JoystickPipeline::new(&ctx);
        let glyph_pipeline = pipelines::GlyphPipeline::new(&ctx, ctx.font_data.clone());
        let image_pipeline = pipelines::UiElementPipeline::new(&ctx);
        let smaa_target = SmaaTarget::new(
            &ctx.device,
            &ctx.queue,
            size.x,
            size.y,
            ctx.color_format,
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
        self.glyph_pipeline = pipelines::GlyphPipeline::new(&self.ctx, self.ctx.font_data.clone());
        self.smaa_target = SmaaTarget::new(
            &self.ctx.device,
            &self.ctx.queue,
            self.ctx.viewport.width,
            self.ctx.viewport.height,
            self.ctx.color_format,
            if self.ctx.settings.smaa {
                SmaaMode::Smaa1X
            } else {
                SmaaMode::Disabled
            },
        );
    }

    pub fn set_viewport<W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle>(
        &mut self,
        window: &W,
        size: Point2<u32>,
        scale_factor: f32,
    ) {
        self.ctx.viewport = viewport::Viewport::new(size.x, size.y, scale_factor, self.ctx.settings.render_scale);
        self.scaling_pipeline.resize(&self.ctx);

        if self.ctx.surface.is_none() {
            self.ctx.surface = Some(unsafe { self.ctx.instance.create_surface(window).expect("Failed to create surface!") });
        }

        if let Some(surface) = &mut self.ctx.surface {
            surface.configure(
                &self.ctx.device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: self.ctx.color_format,
                    width: self.ctx.viewport.width,
                    height: self.ctx.viewport.height,
                    present_mode: wgpu::PresentMode::AutoNoVsync,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![self.ctx.color_format],
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

    pub fn initialize_model(&mut self, gltf_model: &model::GltfModel, name: &str) -> ecs::components::Model {
        let model = pipelines::model::Model::new(&self.ctx, &self.model_pipeline, gltf_model, name);
        let nodes = gltf_model.nodes.clone();
        let animation_times = nodes.animations.iter().map(|(a, b)| (a.clone(), b.total_time)).collect();
        let animation_sound_effects = nodes
            .animations
            .iter()
            .filter(|(_, b)| b.sound_effect.is_some())
            .map(|(a, b)| (a.clone(), b.sound_effect.clone().unwrap()))
            .collect();

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
            key,
            animation_times,
            animation_sound_effects,
        }
    }

    pub fn initialize_particle(&mut self, emitter: pipelines::ParticleEmitter, key: String) {
        self.ctx.emitter_instances.insert(key, emitter);
    }
}

pub fn configure_surface(surface: &wgpu::Surface, device: &wgpu::Device, color_format: wgpu::TextureFormat, size: Point2<u32>) {
    surface.configure(
        device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: color_format,
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![color_format],
        },
    );
}

pub fn load_model(ctx: &Context, path: &str) -> model::GltfModel {
    let bytes = file::read_bytes(path);
    model::GltfModel::new(ctx, bytes.as_slice())
}
