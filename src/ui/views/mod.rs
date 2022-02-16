mod in_game;
mod loading;
mod main_menu;
mod pages;
use super::transition::Transition;
use crate::{
    engine,
    world::{GameState, World},
};
use egui::*;
use egui_wgpu_backend::RenderPass;

pub struct Views {
    loading: loading::Loading,
    in_game: in_game::InGame,
    main_menu: main_menu::MainMenu,
    ui_state: Transition<GameState>,
}

impl Views {
    pub fn new(ctx: &engine::Context, render_pass: &mut RenderPass) -> Self {
        Self {
            loading: loading::Loading::new(ctx, render_pass),
            in_game: in_game::InGame::new(),
            main_menu: main_menu::MainMenu::new(ctx),
            ui_state: Transition::new(GameState::Loading),
        }
    }

    pub fn update(&mut self, ctx: &engine::Context, ui_ctx: &CtxRef, world: &mut World) -> Vec<Rect> {
        self.ui_state.set(world.game_state.clone());
        let opacity = self.ui_state.tick();

        match self.ui_state.state {
            GameState::Loading => self.loading.update(ctx, ui_ctx, opacity),
            GameState::Running => self.in_game.update(ctx, ui_ctx, world, opacity),
            GameState::MainMenu => self.main_menu.update(ctx, ui_ctx, world, opacity),
            _ => vec![],
        }
    }
}
