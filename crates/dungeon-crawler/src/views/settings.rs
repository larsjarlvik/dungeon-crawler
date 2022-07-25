use super::style;
use ui::components::*;
use ui::prelude::*;
use ui::widgets::*;

fn setting(label: &str, control: Box<dyn BaseWidget>) -> Box<NodeWidget> {
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
    ])
}

pub fn settings(ctx: &mut engine::Context, ui_state: &mut ui::State) -> Box<dyn BaseWidget> {
    let render_scale = Slider {
        key: "render_scale".into(),
        value: ctx.settings.render_scale * 100.0,
        max_value: 100.0,
    };

    if let Some(mouse) = ui_state.mouse_down(&render_scale.key) {
        ctx.settings.render_scale = mouse.x;
    }

    let shadow_quality = Slider {
        key: "shadow_quality".into(),
        value: ctx.settings.shadow_map_scale,
        max_value: 4.0,
    };

    if let Some(mouse) = ui_state.mouse_down(&shadow_quality.key) {
        ctx.settings.shadow_map_scale = mouse.x * shadow_quality.max_value;
    }

    NodeWidget::new(FlexboxLayout {
        flex_direction: FlexDirection::Column,
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    })
    .with_children(vec![
        setting("Render scale:", render_scale.draw()),
        setting("Shadow quality:", shadow_quality.draw()),
    ])
}
