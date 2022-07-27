use super::style;
use ui::components::*;
use ui::prelude::*;
use ui::widgets::*;

fn setting(label: &str, control: Box<dyn BaseWidget>, value: f32) -> Box<NodeWidget> {
    NodeWidget::new(FlexboxLayout {
        align_items: AlignItems::Center,
        margin: Rect::from_points(0.0, 0.0, 0.0, style::SM),
        ..Default::default()
    })
    .with_children(vec![
        NodeWidget::new(FlexboxLayout {
            flex_direction: FlexDirection::Column,
            size: Size {
                width: Dimension::Points(150.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        })
        .with_children(vec![TextWidget::new(
            TextData {
                size: style::BODY1,
                text: label.into(),
            },
            Default::default(),
            AlignSelf::FlexStart,
        )]),
        control,
        NodeWidget::new(FlexboxLayout {
            flex_direction: FlexDirection::Column,
            margin: Rect::from_points(style::SL, 0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with_children(vec![TextWidget::new(
            TextData {
                size: style::BODY2,
                text: format!("{:.2}", value),
            },
            Default::default(),
            AlignSelf::FlexStart,
        )]),
    ])
}

fn create_slider<F: FnOnce(f32)>(ui_state: &mut ui::State, key: &str, value: f32, max_value: f32, handle: F) -> Slider {
    if let Some(mouse) = ui_state.mouse_down(&key.to_string()) {
        handle(mouse.x.max(0.0).min(1.0));
    }

    Slider {
        key: key.into(),
        value,
        max_value,
    }
}

pub fn settings(ctx: &mut engine::Context, ui_state: &mut ui::State) -> Box<dyn BaseWidget> {
    let contrast = create_slider(ui_state, "contrast", ctx.settings.contrast, 10.0, |val| {
        ctx.settings.contrast = (val * 20.0).round() / 2.0;
    });

    let render_scale = create_slider(ui_state, "render_scale", ctx.settings.render_scale * 100.0, 100.0, |val| {
        ctx.settings.render_scale = (val * 20.0).round() / 20.0;
    });

    let shadow_quality = create_slider(ui_state, "shadow_quality", ctx.settings.shadow_map_scale, 4.0, |val| {
        ctx.settings.shadow_map_scale = (val * 8.0).round() / 2.0;
    });

    NodeWidget::new(FlexboxLayout {
        flex_direction: FlexDirection::Column,
        margin: Rect::from_points(0.0, 0.0, style::SL, style::SL),
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    })
    .with_children(vec![
        setting("Contrast:", contrast.draw(), contrast.value),
        setting("Render scale:", render_scale.draw(), render_scale.value),
        setting("Shadow quality:", shadow_quality.draw(), shadow_quality.value),
    ])
}
