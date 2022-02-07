mod in_game;
mod main_menu;
use std::time::Instant;

use crate::{
    config,
    world::{GameState, World},
};
pub use in_game::InGame;
pub use main_menu::MainMenu;

pub struct Views {
    in_game: InGame,
    main_menu: MainMenu,
    current_ui_state: GameState,
    last_ui_state_change: Option<Instant>,
}

impl Views {
    pub fn new() -> Self {
        Self {
            in_game: InGame::new(),
            main_menu: MainMenu::new(),
            current_ui_state: GameState::Running,
            last_ui_state_change: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::CtxRef, world: &mut World) -> bool {
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
            GameState::Running => self.in_game.update(ctx, world, opacity),
            GameState::MainMenu => self.main_menu.update(ctx, world, opacity),
            GameState::Terminated => false,
        }
    }
}

fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let x = ((x - edge0) / (edge1 - edge0)).min(1.0).max(0.0);
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}
