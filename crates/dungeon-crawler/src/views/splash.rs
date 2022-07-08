use ui::widgets::NodeWidget;
use ui::{prelude::*, widgets::*};

pub fn splash() -> Box<NodeWidget> {
    NodeWidget::new(
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
                width: Dimension::Points(200.0),
                height: Dimension::Points(200.0),
            },
        )],
    )
}
