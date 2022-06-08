use crate::{
    engine,
    ui::theme::*,
    world::{
        self, components,
        resources::{self, input::UiActionCode},
        GameState, World,
    },
};
use bevy_ecs::prelude::*;
use egui::*;

use super::custom;

pub struct InGame {}

impl InGame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &egui::Context, world: &mut World, opacity: f32) -> Vec<Rect> {
        let fps = { world.components.get_resource::<resources::Fps>().unwrap().fps };
        let mut blocking_elements = vec![];

        Area::new("in_game_top").show(ui_ctx, |ui| {
            apply_theme(ui, opacity);

            custom::Columns::new(&ui, 3)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for health in world
                            .components
                            .query_filtered::<&components::Health, With<components::UserControl>>()
                            .iter(&world.components)
                        {
                            let current = health.current.floor();
                            let display_health = format!("{} / {}", current, health.max);
                            let health_bar = custom::HealthBar::new(health.current / health.max, display_health).desired_width(200.0);
                            ui.add(health_bar);
                        }

                        if ctx.settings.show_fps {
                            ui.label(format!("FPS: {}", fps).to_string());
                        }
                    });
                })
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        for (health, name) in world
                            .components
                            .query_filtered::<(&components::Health, &components::Name), With<components::Display>>()
                            .iter(&world.components)
                        {
                            let health_bar = custom::HealthBar::new(health.current / health.max, name.name.as_str()).desired_width(200.0);
                            ui.add(health_bar);
                        }
                    });
                })
                .show(ui, |ui| {
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

                ui.with_layout(Layout::right_to_left(), |ui| {
                    let mut input = world.components.get_resource_mut::<world::resources::Input>().unwrap();

                    let attack = ui.add_sized([80.0, 80.0], Button::new("Attack"));
                    let pressed = match ui_ctx.input().pointer.hover_pos() {
                        Some(pos) => ui_ctx.input().pointer.any_down() && attack.rect.contains(pos),
                        None => false,
                    };
                    input.set_ui(UiActionCode::Attack, pressed);
                    blocking_elements.push(attack.rect);

                    let health = ui.add_sized([80.0, 80.0], Button::new("Health"));
                    let pressed = match ui_ctx.input().pointer.hover_pos() {
                        Some(pos) => ui_ctx.input().pointer.any_down() && health.rect.contains(pos),
                        None => false,
                    };
                    input.set_ui(UiActionCode::Health, pressed);
                    blocking_elements.push(health.rect);
                });
            });

        blocking_elements
    }
}
