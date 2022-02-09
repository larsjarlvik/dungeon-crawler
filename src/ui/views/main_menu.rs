use crate::{
    ui::theme::*,
    world::{GameState, World},
};
use egui::*;

pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &CtxRef, world: &mut World, opacity: f32) -> Vec<Rect> {
        let menu = CentralPanel::default()
            .frame(default_frame_colored(
                96.0,
                Color32::from_rgba_premultiplied(20, 22, 24, 210),
                opacity,
            ))
            .show(ctx, |ui| {
                apply_theme(ui, opacity);
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(500.0);

                    ui.heading("Dungeon Crawler");
                    ui.add_space(16.0);

                    if ui.button("Resume").clicked() {
                        world.game_state = GameState::Running;
                    }
                    if ui.button("Exit").clicked() {
                        world.game_state = GameState::Terminated;
                    }
                });
            });

        vec![menu.response.rect]
    }
}
