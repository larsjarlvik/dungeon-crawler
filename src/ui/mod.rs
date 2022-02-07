use crate::{config, engine, world::World};
use egui::*;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use std::{iter, time::Instant};
use winit::*;
mod app;
mod theme;
mod views;

pub struct Ui {
    context: CtxRef,
    pub platform: egui_winit::State,
    app: app::App,
    render_pass: RenderPass,
    previous_frame_time: Option<f32>,
}

impl Ui {
    pub fn new(ctx: &engine::Context, window: &winit::window::Window) -> Self {
        let context = CtxRef::default();
        let platform = egui_winit::State::new(window);
        let app = app::App::default();
        let render_pass = RenderPass::new(&ctx.device, config::COLOR_TEXTURE_FORMAT, 1);

        Self {
            context,
            platform,
            app,
            render_pass,
            previous_frame_time: None,
        }
    }

    pub fn handle_event(&mut self, winit_event: &winit::event::WindowEvent) -> bool {
        self.platform.on_event(&self.context, winit_event);
        self.app.blocking
    }

    pub fn update(&mut self, window: &window::Window, world: &mut World) {
        let mut raw_input = self.platform.take_egui_input(window);
        self.context.begin_frame(raw_input.take());

        if self.previous_frame_time.is_none() {
            self.app.setup(&self.context);
        }

        self.app.update(&self.context, world);
    }

    pub fn render(&mut self, ctx: &engine::Context, window: &window::Window, target: &wgpu::TextureView) {
        let egui_start = Instant::now();

        let (_, paint_commands) = self.context.end_frame();
        let paint_jobs = self.context.tessellate(paint_commands);

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

        self.render_pass.update_texture(&ctx.device, &ctx.queue, &self.context.font_image());
        self.render_pass.update_user_textures(&ctx.device, &ctx.queue);
        self.render_pass
            .update_buffers(&ctx.device, &ctx.queue, &paint_jobs, &screen_descriptor);
        self.render_pass
            .execute(&mut encoder, &target, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        ctx.queue.submit(iter::once(encoder.finish()));
    }
}
