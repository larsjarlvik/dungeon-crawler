use cgmath::Vector4;
use ui::prelude::*;
use ui::widgets::*;

pub fn splash() -> Box<dyn BaseWidget> {
    PanelWidget::new(
        AssetData {
            background: Vector4::new(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            ..Default::default()
        },
        vec![AssetWidget::new(
            AssetData {
                asset_id: Some("logo".into()),
                ..Default::default()
            },
            Default::default(),
            Size {
                width: Dimension::Points(250.0),
                height: Dimension::Points(250.0),
            },
        )],
    )
}
