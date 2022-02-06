use egui::*;

pub fn apply_theme(ui: &mut Ui) -> &mut Style {
    let style = ui.style_mut();
    style.spacing.button_padding = vec2(16.0, 16.0);
    style.spacing.item_spacing = vec2(16.0, 16.0);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(24, 24, 24, 50);

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgba_premultiplied(24, 24, 24, 100);
    style.visuals.widgets.hovered.bg_stroke = Stroke {
        width: 1.0,
        color: Color32::from_rgba_premultiplied(48, 48, 48, 100),
    };

    style.visuals.widgets.active.bg_fill = Color32::from_rgba_premultiplied(24, 24, 24, 150);
    style.visuals.widgets.active.bg_stroke = Stroke {
        width: 1.0,
        color: Color32::from_rgba_premultiplied(48, 48, 48, 25),
    };

    style.visuals.widgets.inactive.corner_radius = 8.0;
    style.visuals.widgets.active.corner_radius = 8.0;
    style.visuals.widgets.hovered.corner_radius = 8.0;
    style.visuals.override_text_color = Some(Color32::WHITE);
    style
}

pub fn default_frame(padding: f32) -> Frame {
    egui::Frame {
        margin: vec2(padding, padding),
        fill: Color32::TRANSPARENT,
        ..Default::default()
    }
}

pub fn default_frame_colored(padding: f32, fill: Color32) -> Frame {
    egui::Frame {
        margin: vec2(padding, padding),
        fill,
        ..Default::default()
    }
}
