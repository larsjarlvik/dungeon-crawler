use crate::{
    engine::{self, settings::Settings},
    ui::theme::*,
    world::{GameState, World},
};
use egui::*;

enum MenuState {
    None,
    Settings,
}

pub struct MainMenu {
    menu_state: MenuState,
    settings: Settings,
}

impl MainMenu {
    pub fn new(ctx: &engine::Context) -> Self {
        Self {
            menu_state: MenuState::None,
            settings: ctx.settings.clone(),
        }
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &CtxRef, world: &mut World, opacity: f32) -> Vec<Rect> {
        let vw = ctx.viewport.width as f32;

        let menu = SidePanel::left("main_menu")
            .min_width(vw * 0.25)
            .max_width(vw * 0.25)
            .resizable(false)
            .frame(default_frame_colored(
                vw * 0.05,
                Color32::from_rgba_premultiplied(10, 10, 10, 210),
                opacity,
            ))
            .show(ui_ctx, |ui| {
                apply_theme(ui, opacity);
                ui.vertical_centered(|ui| {
                    ui.heading("Dungeon Crawler");
                    ui.add_space(16.0);
                });
                ui.vertical_centered_justified(|ui| {
                    self.main_menu(ctx, ui, world);
                });
            });

        let center = CentralPanel::default()
            .frame(default_frame_colored(
                10.0,
                Color32::from_rgba_premultiplied(20, 22, 24, 210),
                opacity,
            ))
            .show(ui_ctx, |ui| {
                apply_theme(ui, opacity);

                ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(vw * 0.07);
                        ui.vertical(|ui| {
                            let menu_state = &mut self.menu_state;

                            ui.add_space(vw * 0.07);
                            match menu_state {
                                MenuState::Settings => {
                                    self.settings(world, ui);
                                }
                                _ => {}
                            };
                        });
                    })
                });
            });

        vec![menu.response.rect, center.response.rect]
    }

    fn main_menu(&mut self, ctx: &engine::Context, ui: &mut Ui, world: &mut World) {
        if ui.button("Settings").clicked() {
            self.settings = ctx.settings.clone();
            self.menu_state = MenuState::Settings;
        }
        if ui.button("Resume").clicked() {
            self.menu_state = MenuState::None;
            world.game_state = GameState::Running;
        }
        if ui.button("Exit").clicked() {
            world.game_state = GameState::Terminated;
        }
    }

    fn settings(&mut self, world: &mut World, ui: &mut Ui) {
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
            if ui.button("Cancel").clicked() {
                self.menu_state = MenuState::None;
            };

            if ui.button("Apply").clicked() {
                self.settings.store();
                self.menu_state = MenuState::None;
                world.game_state = GameState::Reload;
            };

            ui.add_space(40.0);
            if ui.button("Reset").clicked() {
                self.settings = Settings::default();
                self.settings.store();
                self.menu_state = MenuState::None;
                world.game_state = GameState::Reload;
            };
        });
    }
}
