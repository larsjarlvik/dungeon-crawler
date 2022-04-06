use crate::{config, engine, world::World};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use std::{iter, time::Instant};
use winit::*;
mod app;
mod theme;
mod transition;
mod utils;
mod views;

pub struct Ui {
    ctx: egui::Context,
    pub platform: egui_winit::State,
    app: app::App,
    render_pass: RenderPass,
    previous_frame_time: Option<f32>,
    scale_factor: f32,
}

impl Ui {
    pub fn new(ctx: &engine::Context, window: &winit::window::Window) -> Self {
        let ui_ctx = egui::Context::default();
        let platform = egui_winit::State::new(4096, window);
        let mut render_pass = RenderPass::new(&ctx.device, config::COLOR_TEXTURE_FORMAT, 1);
        let app = app::App::new(&ctx, &ui_ctx, &mut render_pass);

        Self {
            ctx: ui_ctx,
            platform,
            app,
            render_pass,
            previous_frame_time: None,
            scale_factor: window.scale_factor() as f32,
        }
    }

    pub fn handle_event(&mut self, winit_event: &winit::event::WindowEvent) {
        self.platform.on_event(&self.ctx, winit_event);
    }

    pub fn update(&mut self, window: &window::Window, ctx: &engine::Context, world: &mut World) {
        let mut raw_input = self.platform.take_egui_input(window);
        self.ctx.begin_frame(raw_input.take());
        self.app.update(ctx, &self.ctx, world);
    }

    pub fn render(&mut self, ctx: &engine::Context, window: &window::Window, target: &wgpu::TextureView) {
        let egui_start = Instant::now();

        let output = self.ctx.end_frame();
        let paint_jobs = self.ctx.tessellate(output.shapes);

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
            .add_textures(&ctx.device, &ctx.queue, &output.textures_delta)
            .expect("Failed to add UI textures!");
        self.render_pass
            .update_buffers(&ctx.device, &ctx.queue, &paint_jobs, &screen_descriptor);
        self.render_pass
            .execute(&mut encoder, &target, &paint_jobs, &screen_descriptor, None)
            .unwrap();
        self.render_pass
            .remove_textures(output.textures_delta)
            .expect("Failed to remove UI textures!");

        ctx.queue.submit(iter::once(encoder.finish()));
    }

    pub fn is_blocking(&self, position: cgmath::Point2<f32>) -> bool {
        let p = position / self.scale_factor;
        self.app
            .blocking_elements
            .iter()
            .any(|b| b.min.x <= p.x && b.max.x >= p.x && b.min.y <= p.y && b.max.y >= p.y)
    }
}
