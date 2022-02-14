use crate::{
    engine::{self, settings::Settings},
    ui::theme::*,
    world::{GameState, World},
};
use egui::*;

enum MenuState {
    MainMenu,
    Settings,
}

pub struct MainMenu {
    menu_state: MenuState,
    settings: Settings,
}

impl MainMenu {
    pub fn new(ctx: &engine::Context) -> Self {
        Self {
            menu_state: MenuState::MainMenu,
            settings: ctx.settings.clone(),
        }
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &CtxRef, world: &mut World, opacity: f32) -> Vec<Rect> {
        let menu = CentralPanel::default()
            .frame(default_frame_colored(
                48.0,
                Color32::from_rgba_premultiplied(20, 22, 24, 210),
                opacity,
            ))
            .show(ui_ctx, |ui| {
                apply_theme(ui, opacity);
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(500.0);

                    let menu_state = &mut self.menu_state;
                    match menu_state {
                        MenuState::MainMenu => {
                            self.main_menu(ctx, ui, world);
                        }
                        MenuState::Settings => {
                            self.settings(world, ui);
                        }
                    }
                });
            });

        vec![menu.response.rect]
    }

    fn main_menu(&mut self, ctx: &engine::Context, ui: &mut Ui, world: &mut World) {
        ui.heading("Dungeon Crawler");
        ui.add_space(16.0);

        if ui.button("Settings").clicked() {
            self.settings = ctx.settings.clone();
            self.menu_state = MenuState::Settings;
        }
        if ui.button("Resume").clicked() {
            world.game_state = GameState::Running;
        }
        if ui.button("Exit").clicked() {
            world.game_state = GameState::Terminated;
        }
    }

    fn settings(&mut self, world: &mut World, ui: &mut Ui) {
        ui.heading("Settings");
        ui.add_space(16.0);

        Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::Min)
            .with_main_wrap(false)
            .with_cross_justify(true);

        ui.horizontal(|ui| {
            ui.label("Brightness:");
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add_sized([35.0, 24.0], Label::new(format!("{:.2}", self.settings.brightness)));
                ui.add(Slider::new(&mut self.settings.brightness, -0.5..=0.5).show_value(false));
            });
        });

        ui.horizontal(|ui| {
            ui.label("Contrast:");
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add_sized([35.0, 24.0], Label::new(format!("{:.2}", self.settings.contrast)));
                ui.add(Slider::new(&mut self.settings.contrast, 0.0..=10.0).show_value(false));
            });
        });

        ui.horizontal(|ui| {
            ui.label("Render Scale:");
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add_sized([35.0, 24.0], Label::new(format!("{:.2}", self.settings.render_scale)));
                ui.add(Slider::new(&mut self.settings.render_scale, 0.1..=1.0).show_value(false));
            });
        });

        ui.horizontal(|ui| {
            ui.label("Shadow quality:");
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add_sized([35.0, 24.0], Label::new(format!("{:.2}", self.settings.shadow_map_scale)));
                ui.add(Slider::new(&mut self.settings.shadow_map_scale, 0.5..=4.0).show_value(false));
            });
        });

        ui.add_space(ui.spacing().item_spacing.y);
        ui.columns(2, |columns| {
            columns[0].vertical_centered_justified(|ui| {
                if ui.button("Cancel").clicked() {
                    self.menu_state = MenuState::MainMenu;
                };
            });
            columns[1].vertical_centered_justified(|ui| {
                if ui.button("Apply").clicked() {
                    self.settings.store();
                    world.game_state = GameState::Reload;
                };
            });
        });
    }
}
