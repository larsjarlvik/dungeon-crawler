use crate::{
    ui::theme::*,
    world::{resources, GameState, World},
};
use egui::*;
use specs::WorldExt;

pub struct InGame {}

impl InGame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &CtxRef, world: &mut World) -> bool {
        let fps = { world.components.read_resource::<resources::Fps>().fps };

        let top = TopBottomPanel::top("in_game_top").frame(default_frame(16.0)).show(ctx, |ui| {
            apply_theme(ui);

            ui.horizontal(|ui| {
                ui.label(format!("FPS: {}", fps).to_string());

                ui.with_layout(Layout::right_to_left(), |ui| {
                    if ui.add_sized([50.0, 50.0], Button::new("\u{2630}")).clicked() {
                        world.game_state = GameState::MainMenu;
                    }
                });
            });
        });

        let bottom = TopBottomPanel::bottom("in_game_bottom").frame(default_frame(32.0)).show(ctx, |ui| {
            apply_theme(ui);

            ui.with_layout(Layout::right_to_left(), |_ui| {
                // TODO: Action buttons
            });
        });

        bottom.response.hovered() || top.response.hovered()
    }
}
