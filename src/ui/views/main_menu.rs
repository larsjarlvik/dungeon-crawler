use super::pages;
use crate::{
    engine::{self},
    ui::{theme::*, transition::Transition},
    world::{GameState, World},
};
use egui::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MenuItem {
    None,
    Settings,
}

pub struct MainMenu {
    menu_state: Transition<MenuItem>,
    settings: pages::Settings,
}

impl MainMenu {
    pub fn new(ctx: &engine::Context) -> Self {
        Self {
            menu_state: Transition::new(MenuItem::None),
            settings: pages::Settings::new(ctx),
        }
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &egui::Context, world: &mut World, opacity: f32) -> Vec<Rect> {
        let vw = ctx.viewport.width as f32 / ui_ctx.pixels_per_point();
        let center_opacity = self.menu_state.tick();

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
                    self.main_menu(ui, world, opacity);
                });
            });

        let center = CentralPanel::default()
            .frame(default_frame_colored(
                10.0,
                Color32::from_rgba_premultiplied(20, 22, 24, 210),
                opacity,
            ))
            .show(ui_ctx, |ui| {
                apply_theme(ui, center_opacity.min(opacity));

                ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(vw * 0.06);
                        ui.vertical(|ui| {
                            ui.add_space(vw * 0.06);

                            self.menu_state.optional_set(match &self.menu_state.state {
                                MenuItem::Settings => self.settings.update(world, ui),
                                _ => None,
                            });
                        });
                    })
                });
            });

        vec![menu.response.rect, center.response.rect]
    }

    fn main_menu(&mut self, ui: &mut Ui, world: &mut World, opacity: f32) {
        self.menu_button(ui, MenuItem::Settings, "Settings", opacity, || {});
        self.menu_button(ui, MenuItem::None, "Resume", opacity, || {
            world.game_state = GameState::Running;
        });
        self.menu_button(ui, MenuItem::None, "Exit", opacity, || {
            world.game_state = GameState::Terminated;
        });
    }

    fn menu_button<F: FnOnce()>(&mut self, ui: &mut Ui, menu: MenuItem, text: &str, opacity: f32, f: F) {
        ui.scope(|ui| {
            if menu == self.menu_state.state && menu != MenuItem::None {
                apply_active(ui, opacity);
            }

            if ui.button(text).clicked() {
                self.menu_state.set(menu);
                f();
            }
        });
    }
}
