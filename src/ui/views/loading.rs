use crate::{
    engine,
    ui::{theme::*, utils},
};
use egui::*;
use egui_wgpu_backend::RenderPass;

pub struct Loading {
    texture_id: TextureId,
}

impl Loading {
    pub fn new(ctx: &engine::Context, render_pass: &mut RenderPass) -> Self {
        let texture_id = utils::load_image(ctx, render_pass, "icon.png");
        Self { texture_id }
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &CtxRef, opacity: f32) -> Vec<Rect> {
        let vh = ctx.viewport.height as f32 / ui_ctx.pixels_per_point();

        let menu = CentralPanel::default()
            .frame(default_frame_colored(
                vh / 2.0 - 150.0,
                Color32::from_rgba_premultiplied(0, 0, 0, 255),
                opacity,
            ))
            .show(ui_ctx, |ui| {
                apply_theme(ui, opacity);
                ui.vertical_centered_justified(|ui| {
                    if opacity == 1.0 {
                        ui.image(self.texture_id, vec2(300.0, 300.0));
                    }
                });
            });

        vec![menu.response.rect]
    }
}
