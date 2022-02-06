use super::views;
use crate::world::{GameState, World};
use egui::*;

pub struct App {
    in_game: views::InGame,
    main_menu: views::MainMenu,
    pub blocking: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            blocking: false,
            in_game: views::InGame::new(),
            main_menu: views::MainMenu::new(),
        }
    }
}

impl App {
    pub fn setup(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "font".to_owned(),
            FontData::from_owned(include_bytes!("./exo2-medium.ttf").to_vec()),
        );
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "font".to_owned());

        fonts.family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 28.0));
        fonts.family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 18.0));
        fonts.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 18.0));
        ctx.set_fonts(fonts);
    }

    pub fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame, world: &mut World) {
        self.blocking = match world.game_state {
            GameState::Running => self.in_game.update(ctx, world),
            GameState::MainMenu => self.main_menu.update(ctx, world),
            GameState::Terminated => false,
        };

        frame.set_window_size(ctx.used_size());
    }
}
