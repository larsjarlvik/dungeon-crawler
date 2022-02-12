use crate::{
    engine,
    ui::{theme::*, utils},
};
use egui::*;
use egui_wgpu_backend::RenderPass;

pub struct Loading {
    texture_id: TextureId,
    height: f32,
}

impl Loading {
    pub fn new(ctx: &engine::Context, render_pass: &mut RenderPass) -> Self {
        let texture_id = utils::load_image(ctx, render_pass, "icon.png");
        Self {
            texture_id,
            height: ctx.viewport.height as f32,
        }
    }

    pub fn update(&mut self, ctx: &CtxRef, opacity: f32) -> Vec<Rect> {
        let menu = CentralPanel::default()
            .frame(default_frame_colored(
                self.height / 2.0 - 170.0,
                Color32::from_rgba_premultiplied(0, 0, 0, 255),
                opacity,
            ))
            .show(ctx, |ui| {
                apply_theme(ui, opacity);
                ui.vertical_centered_justified(|ui| {
                    ui.image(self.texture_id, vec2(300.0, 300.0));
                    ui.label("Loading...");
                });
            });

        vec![menu.response.rect]
    }
}
