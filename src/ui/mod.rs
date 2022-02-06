use self::repaint_signal::RepaintSignal;
use crate::{config, engine};
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::*;
use epi::App;
use std::{iter, sync::Arc, time::Instant};
use winit::*;
pub mod repaint_signal;

pub struct Ui {
    pub platform: Platform,
    app: egui_demo_lib::WrapApp,
    start_time: Instant,
    render_pass: RenderPass,
    previous_frame_time: Option<f32>,
}

impl Ui {
    pub fn new(ctx: &engine::Context) -> Self {
        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: 800,
            physical_height: 600,
            scale_factor: 1.0,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        let app = egui_demo_lib::WrapApp::default();
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

    pub fn render(
        &mut self,
        ctx: &engine::Context,
        window: &window::Window,
        repaint_signal: &Arc<RepaintSignal>,
        target: &wgpu::TextureView,
    ) {
        self.platform.update_time(self.start_time.elapsed().as_secs_f64());
        let egui_start = Instant::now();
        self.platform.begin_frame();
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

        // Draw the demo application.
        self.app.update(&self.platform.context(), &mut frame);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self.platform.end_frame(Some(&window));
        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
        self.previous_frame_time = Some(frame_time);

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("UI") });

        // Upload all resources for the GPU.
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
            .execute(&mut encoder, &target, &paint_jobs, &screen_descriptor, Some(wgpu::Color::BLACK))
            .unwrap();

        ctx.queue.submit(iter::once(encoder.finish()));
    }
}
