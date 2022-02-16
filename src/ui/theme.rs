use egui::*;

pub fn apply_theme(ui: &mut Ui, opacity: f32) -> &mut Style {
    let style = ui.style_mut();

    style.spacing.button_padding = vec2(16.0, 16.0);
    style.spacing.item_spacing = vec2(16.0, 16.0);
    style.spacing.slider_width = 250.0;
    style.spacing.icon_width = 22.0;

    style.visuals.override_text_color = Some(Color32::WHITE.linear_multiply(opacity));
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(0, 0, 0, 100).linear_multiply(opacity);
    style.visuals.widgets.inactive.bg_stroke = Stroke {
        width: 2.0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 150).linear_multiply(opacity),
    };

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgba_premultiplied(0, 0, 0, 150).linear_multiply(opacity);
    style.visuals.widgets.hovered.bg_stroke = Stroke {
        width: 2.0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 200).linear_multiply(opacity),
    };

    style.visuals.widgets.active.bg_fill = style.visuals.widgets.hovered.bg_fill;
    style.visuals.widgets.active.bg_stroke = style.visuals.widgets.hovered.bg_stroke;

    style.visuals.widgets.inactive.corner_radius = 8.0;
    style.visuals.widgets.active.corner_radius = 8.0;
    style.visuals.widgets.hovered.corner_radius = 8.0;

    style
}

pub fn apply_active(ui: &mut Ui, opacity: f32) {
    let style = ui.visuals_mut();

    style.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(120, 0, 0, 100).linear_multiply(opacity);
    style.widgets.inactive.bg_stroke = Stroke {
        width: 2.0,
        color: Color32::from_rgba_premultiplied(120, 0, 0, 150).linear_multiply(opacity),
    };

    style.widgets.hovered.bg_fill = Color32::from_rgba_premultiplied(150, 0, 0, 100).linear_multiply(opacity);
    style.widgets.hovered.bg_stroke = Stroke {
        width: 2.0,
        color: Color32::from_rgba_premultiplied(150, 0, 0, 150).linear_multiply(opacity),
    };

    style.widgets.active.bg_fill = style.widgets.hovered.bg_fill;
    style.widgets.active.bg_stroke = style.widgets.hovered.bg_stroke;
}

pub fn default_frame(padding: f32) -> Frame {
    Frame {
        margin: vec2(padding, padding),
        fill: Color32::TRANSPARENT,
        ..Default::default()
    }
}

pub fn default_frame_colored(padding: f32, fill: Color32, opacity: f32) -> Frame {
    Frame {
        margin: vec2(padding, padding),
        fill: fill.linear_multiply(opacity),
        ..Default::default()
    }
}
