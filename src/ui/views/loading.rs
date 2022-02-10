use crate::ui::theme::*;
use egui::*;

pub struct Loading {}

impl Loading {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &CtxRef, opacity: f32) -> Vec<Rect> {
        let menu = CentralPanel::default()
            .frame(default_frame_colored(
                120.0,
                Color32::from_rgba_premultiplied(0, 0, 0, 255),
                opacity,
            ))
            .show(ctx, |ui| {
                apply_theme(ui, opacity);
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Dungeon Crawler");
                    ui.add_space(28.0);
                    ui.label("Loading...");
                });
            });

        vec![menu.response.rect]
    }
}
