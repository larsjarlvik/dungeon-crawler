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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
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
                width: Dimension::Percent(0.5),
                height: Dimension::Percent(0.5),
            },
            max_size: Size {
                width: Dimension::Points(250.0),
                height: Dimension::Points(250.0),
            },
            ..Default::default()
        },
    )])
}
