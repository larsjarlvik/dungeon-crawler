mod in_game;
mod loading;
mod main_menu;
use crate::{
    config, engine,
    world::{GameState, World},
};
use egui::*;
use egui_wgpu_backend::RenderPass;
use std::time::Instant;

pub struct Views {
    loading: loading::Loading,
    in_game: in_game::InGame,
    main_menu: main_menu::MainMenu,
    current_ui_state: GameState,
    last_ui_state_change: Option<Instant>,
}

impl Views {
    pub fn new(ctx: &engine::Context, render_pass: &mut RenderPass) -> Self {
        Self {
            loading: loading::Loading::new(ctx, render_pass),
            in_game: in_game::InGame::new(),
            main_menu: main_menu::MainMenu::new(ctx),
            current_ui_state: GameState::Loading,
            last_ui_state_change: None,
        }
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &CtxRef, world: &mut World) -> Vec<Rect> {
        if world.game_state != self.current_ui_state {
            match self.last_ui_state_change {
                Some(ui) => {
                    if ui.elapsed().as_secs_f32() > config::UI_TRANSITION_TIME * 0.5 {
                        self.current_ui_state = world.game_state.clone();
                    }
                }
                None => {
                    self.last_ui_state_change = Some(Instant::now());
                }
            };
        } else if let Some(ui) = self.last_ui_state_change {
            if ui.elapsed().as_secs_f32() > config::UI_TRANSITION_TIME {
                self.last_ui_state_change = None;
            }
        }

        let opacity = match self.last_ui_state_change {
            Some(ui) => {
                let e = ui.elapsed().as_secs_f32() / config::UI_TRANSITION_TIME;
                (smootherstep(0.0, 1.0, e) * 2.0 - 1.0).abs().min(1.0)
            }
            None => 1.0,
        };

        match self.current_ui_state {
            GameState::Loading => self.loading.update(ui_ctx, opacity),
            GameState::Running => self.in_game.update(ctx, ui_ctx, world, opacity),
            GameState::MainMenu => self.main_menu.update(ctx, ui_ctx, world, opacity),
            _ => vec![],
        }
    }
}

fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let x = ((x - edge0) / (edge1 - edge0)).min(1.0).max(0.0);
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}
