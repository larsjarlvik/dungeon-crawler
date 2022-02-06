use egui::*;

pub struct App {
    counter: i32,
}

impl Default for App {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl epi::App for App {
    fn setup(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (egui::FontFamily::Monospace, 132.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Button, (egui::FontFamily::Proportional, 132.0));
        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
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

        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Dungeon Crawler"
    }
}
