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

    pub fn update(&mut self, ctx: &CtxRef, world: &mut World) -> bool {
        let menu = CentralPanel::default()
            .frame(default_frame_colored(96.0, Color32::from_rgba_premultiplied(0, 0, 0, 150)))
            .show(ctx, |ui| {
                apply_theme(ui);

                ui.vertical_centered_justified(|ui| {
                    ui.heading("Menu");

                    if ui.button("Resume").clicked() {
                        world.game_state = GameState::Running;
                    }
                    if ui.button("Exit").clicked() {
                        world.game_state = GameState::Terminated;
                    }
                });
            });

        menu.response.hovered()
    }
}
