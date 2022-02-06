use egui::*;

pub fn apply_theme(ui: &mut Ui) {
    let style = ui.style_mut();
    style.spacing.button_padding = vec2(16.0, 16.0);
    style.visuals.widgets.inactive.bg_fill = Color32::DARK_BLUE;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(2.0, Color32::WHITE);
    style.visuals.widgets.inactive.corner_radius = 0.0;
    style.visuals.override_text_color = Some(Color32::WHITE);
}

pub fn default_frame(padding: f32) -> Frame {
    egui::Frame {
        margin: vec2(padding, padding),
        fill: Color32::TRANSPARENT,
        ..Default::default()
    }
}
