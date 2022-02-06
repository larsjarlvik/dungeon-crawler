use self::repaint_signal::RepaintSignal;
use crate::{config, engine, world::World};
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
mod egui_winit_platform;
use egui_winit_platform::*;
use std::{iter, sync::Arc, time::Instant};
use winit::*;
mod app;
pub mod repaint_signal;
mod theme;
mod views;

pub struct Ui {
    pub platform: Platform,
    app: app::App,
    start_time: Instant,
    render_pass: RenderPass,
    previous_frame_time: Option<f32>,
}

impl Ui {
    pub fn new(ctx: &engine::Context, window: &winit::window::Window) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: ctx.viewport.width,
            physical_height: ctx.viewport.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        let app = app::App::default();
        let start_time = Instant::now();
        let render_pass = RenderPass::new(&ctx.device, config::COLOR_TEXTURE_FORMAT, 1);

        Self {
            platform,
            app,
            start_time,
            render_pass,
            previous_frame_time: None,
        }
    }

    pub fn handle_event(&mut self, winit_event: &event::Event<repaint_signal::Event>) -> bool {
        self.platform.handle_event(&winit_event);
        self.app.blocking
    }

    pub fn update(&mut self, window: &window::Window, repaint_signal: &Arc<RepaintSignal>, world: &mut World) {
        let app_output = epi::backend::AppOutput::default();
        let mut frame = epi::Frame::new(epi::backend::FrameData {
            info: epi::IntegrationInfo {
                name: "egui_example",
                web_info: None,
                cpu_usage: self.previous_frame_time,
                native_pixels_per_point: Some(window.scale_factor() as f32),
                prefer_dark_mode: None,
            },
            output: app_output,
            repaint_signal: repaint_signal.clone(),
        });

        self.platform.begin_frame();
        let ctx = self.platform.context();

        if self.previous_frame_time.is_none() {
            self.app.setup(&ctx, &mut frame, None);
        }

        self.app.update(&ctx, &mut frame, world);
    }

    pub fn render(&mut self, ctx: &engine::Context, window: &window::Window, target: &wgpu::TextureView) {
        self.platform.update_time(self.start_time.elapsed().as_secs_f64());
        let egui_start = Instant::now();
        self.platform.begin_frame();

        let (_output, paint_commands) = self.platform.end_frame(Some(&window));
        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
        self.previous_frame_time = Some(frame_time);

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("UI") });

        let screen_descriptor = ScreenDescriptor {
            physical_width: ctx.viewport.width,
            physical_height: ctx.viewport.height,
            scale_factor: window.scale_factor() as f32,
        };

        self.render_pass
            .update_texture(&ctx.device, &ctx.queue, &self.platform.context().font_image());
        self.render_pass.update_user_textures(&ctx.device, &ctx.queue);
        self.render_pass
            .update_buffers(&ctx.device, &ctx.queue, &paint_jobs, &screen_descriptor);
        self.render_pass
            .execute(&mut encoder, &target, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        ctx.queue.submit(iter::once(encoder.finish()));
        self.platform.end_frame(Some(window));
    }
}
