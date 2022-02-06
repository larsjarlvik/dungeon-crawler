use egui::*;

pub struct App {
    counter: i32,
    pub blocking: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0,
            blocking: false,
        }
    }
}

impl epi::App for App {
    fn setup(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let font_definitions = {
            use egui::paint::fonts::{FontFamily, TextStyle};
            let family = FontFamily::Mononoki;

            let mut def = egui::paint::FontDefinitions::default();
            def.fonts.insert(TextStyle::Body, (family, 32));
            def.fonts.insert(TextStyle::Button, (family, 32));
            def.fonts.insert(TextStyle::Heading, (family, 32));
            def.fonts.insert(TextStyle::Monospace, (family, 32));
            def
        };
        ctx.set_fonts(font_definitions);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let panel = egui::TopBottomPanel::bottom("bottom_panel")
            .frame(egui::Frame {
                margin: vec2(32.0, 32.0),
                fill: Color32::TRANSPARENT,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let style = ui.style_mut();
                style.spacing.button_padding = vec2(16.0, 16.0);

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("+").clicked() {
                        self.counter += 1;
                    }
                    ui.label(format!("COUNT: {}", self.counter).to_string());
                    if ui.button("-").clicked() {
                        self.counter -= 1;
                    }
                });
            });

        self.blocking = panel.response.hovered();
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Dungeon Crawler"
    }
}
