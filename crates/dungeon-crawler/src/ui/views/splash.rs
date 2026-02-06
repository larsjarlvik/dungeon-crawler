use cgmath::Vector4;
use ui::prelude::*;
use ui::widgets::*;

pub fn splash() -> Box<dyn BaseWidget> {
    DisplayWidget::new(
        DisplayWidgetProps {
            background: Vector4::new(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        Style {
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            ..Default::default()
        },
    )
    .with_children(vec![DisplayWidget::new(
        DisplayWidgetProps {
            asset_id: Some("logo".into()),
            ..Default::default()
        },
        Style {
            aspect_ratio: Some(1.0),
            size: Size {
                width: percent(0.5),
                height: percent(0.5),
            },
            max_size: Size {
                width: length(250.0),
                height: length(250.0),
            },
            ..Default::default()
        },
    )])
}
