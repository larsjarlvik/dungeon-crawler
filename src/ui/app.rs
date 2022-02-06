use super::views;
use egui::*;

enum Views {
    InGame(views::InGame),
}

pub struct App {
    view: Views,
    pub blocking: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            blocking: false,
            view: Views::InGame(views::InGame::new()),
        }
    }
}

impl App {
    pub fn setup(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        println!("setup");
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

        fonts.family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 18.0));
        fonts.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 18.0));
        ctx.set_fonts(fonts);
    }

    pub fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame, components: &specs::World) {
        self.blocking = match &mut self.view {
            Views::InGame(in_game) => in_game.update(ctx, components),
        };

        frame.set_window_size(ctx.used_size());
    }
}
