use crate::{ui::theme::*, world::resources};
use egui::*;
use specs::WorldExt;

pub struct InGame {
    counter: i32,
}

impl InGame {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn update(&mut self, ctx: &CtxRef, components: &specs::World) -> bool {
        let fps = components.read_resource::<resources::Fps>();

        egui::TopBottomPanel::top("in_game_top").frame(default_frame(32.0)).show(ctx, |ui| {
            apply_theme(ui);

            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.label(format!("FPS: {}", fps.fps).to_string());
            });
        });

        let bottom = egui::TopBottomPanel::bottom("in_game_bottom")
            .frame(default_frame(32.0))
            .show(ctx, |ui| {
                apply_theme(ui);

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.add_sized([50.0, 50.0], Button::new("+")).clicked() {
                        self.counter += 1;
                    }

                    ui.label(format!("COUNT: {}", self.counter).to_string());

                    if ui.add_sized([50.0, 50.0], Button::new("-")).clicked() {
                        self.counter -= 1;
                    }
                });
            });

        bottom.response.hovered()
    }
}
