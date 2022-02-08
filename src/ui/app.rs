use super::views::{self, Views};
use crate::world::World;
use egui::*;

pub struct App {
    pub views: Views,
    pub blocking_elements: Vec<Rect>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            views: views::Views::new(),
            blocking_elements: vec![],
        }
    }
}

impl App {
    pub fn setup(&mut self, ctx: &egui::CtxRef) {
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

    pub fn update(&mut self, ctx: &egui::CtxRef, world: &mut World) {
        self.blocking_elements = self.views.update(ctx, world);
    }
}
