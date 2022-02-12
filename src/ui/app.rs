use super::views::{self, Views};
use crate::{engine, utils, world::World};
use egui::*;
use egui_wgpu_backend::RenderPass;

pub struct App {
    pub views: Views,
    pub blocking_elements: Vec<Rect>,
}

impl App {
    pub fn new(ctx: &engine::Context, ui_ctx: &egui::CtxRef, render_pass: &mut RenderPass) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts
            .font_data
            .insert("font".to_owned(), FontData::from_owned(utils::read_bytes("exo2-medium.ttf")));
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "font".to_owned());

        fonts.family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 28.0));
        fonts.family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 18.0));
        fonts.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 18.0));

        ui_ctx.set_fonts(fonts);

        Self {
            views: views::Views::new(ctx, render_pass),
            blocking_elements: vec![],
        }
    }

    pub fn update(&mut self, ctx: &egui::CtxRef, world: &mut World) {
        self.blocking_elements = self.views.update(ctx, world);
    }
}
