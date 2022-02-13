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

    pub fn update(&mut self, ui_ctx: &CtxRef, world: &mut World, opacity: f32) -> Vec<Rect> {
        let fps = { world.components.read_resource::<resources::Fps>().fps };
        let mut blocking_elements = vec![];

        TopBottomPanel::top("in_game_top").frame(default_frame(16.0)).show(ui_ctx, |ui| {
            apply_theme(ui, opacity);

            ui.horizontal(|ui| {
                ui.label(format!("FPS: {}", fps).to_string());

                ui.with_layout(Layout::right_to_left(), |ui| {
                    let menu_button = ui.add_sized([50.0, 50.0], Button::new("\u{2630}"));
                    if menu_button.clicked() {
                        world.game_state = GameState::MainMenu;
                    }

                    blocking_elements.push(menu_button.rect);
                });
            });
        });

        TopBottomPanel::bottom("in_game_bottom")
            .frame(default_frame(32.0))
            .show(ui_ctx, |ui| {
                apply_theme(ui, opacity);

                ui.with_layout(Layout::right_to_left(), |_ui| {
                    // TODO: Action buttons
                });
            });

        blocking_elements
    }
}
