use crate::{
    engine,
    ui::views::main_menu::MenuItem,
    world::{GameState, World},
};
use egui::*;

pub struct Settings {
    pub settings: engine::Settings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            settings: Default::default(),
        }
    }
}

impl Settings {
    pub fn new(ctx: &engine::Context) -> Self {
        Self {
            settings: ctx.settings.clone(),
        }
    }

    pub fn update(&mut self, world: &mut World, ui: &mut Ui) -> Option<MenuItem> {
        let mut new_menu_state = None;

        ui.vertical_centered_justified(|ui| {
            egui::Grid::new("settings_grid")
                .num_columns(2)
                .spacing([30.0, 20.0])
                .show(ui, |ui| {
                    ui.label("Brightness:");
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut self.settings.brightness, -0.5..=0.5).show_value(false));
                        ui.label(format!("{:.2}", self.settings.brightness));
                    });
                    ui.end_row();

                    ui.label("Contrast:");
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut self.settings.contrast, 0.0..=10.0).show_value(false));
                        ui.label(format!("{:.2}", self.settings.contrast));
                    });
                    ui.end_row();

                    ui.label("Bloom:");
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut self.settings.bloom, 0.0..=10.0).show_value(false));
                        ui.label(format!("{:.2}", self.settings.bloom));
                    });
                    ui.end_row();

                    ui.label("Render Scale:");
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut self.settings.render_scale, 0.1..=1.0).show_value(false));
                        ui.label(format!("{:.2}", self.settings.render_scale));
                    });
                    ui.end_row();

                    ui.label("Shadow quality:");
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut self.settings.shadow_map_scale, 0.5..=4.0).show_value(false));
                        ui.label(format!("{:.2}", self.settings.shadow_map_scale));
                    });
                    ui.end_row();
                    ui.checkbox(&mut self.settings.show_fps, "Show FPS");
                    ui.end_row();
                });
        });

        ui.set_min_height(100.0);
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                self.settings.store();
                new_menu_state = Some(MenuItem::None);
                world.game_state = GameState::Reload;
            };

            if ui.button("Cancel").clicked() {
                self.settings = self.settings.clone();
                new_menu_state = Some(MenuItem::None);
            };

            ui.add_space(40.0);
            if ui.button("Reset").clicked() {
                self.settings = engine::Settings::default();
                self.settings.store();
                new_menu_state = Some(MenuItem::None);
                world.game_state = GameState::Reload;
            };
        });

        new_menu_state
    }
}
