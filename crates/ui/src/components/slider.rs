use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Slider {
    pub key: String,
    pub value: f32,
    pub max_value: f32,
}

impl Slider {
    pub fn draw(&self) -> Box<PanelWidget> {
        PanelWidget::new(
            AssetData {
                key: self.key.clone().into(),
                visible: false,
                ..Default::default()
            },
            FlexboxLayout {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Points(24.0),
                },
                ..Default::default()
            },
        )
        .with_children(vec![PanelWidget::new(
            AssetData {
                background: Vector4::new(0.0, 0.0, 0.0, 0.6),
                ..Default::default()
            },
            FlexboxLayout {
                size: Size {
                    width: Dimension::Points(260.0),
                    height: Dimension::Points(4.0),
                },
                ..Default::default()
            },
        )
        .with_children(vec![
            PanelWidget::new(
                AssetData {
                    background: Vector4::new(1.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                FlexboxLayout {
                    size: Size {
                        width: Dimension::Percent(self.value / self.max_value),
                        height: Dimension::Points(4.0),
                    },
                    ..Default::default()
                },
            ),
            PanelWidget::new(
                AssetData {
                    background: Vector4::new(1.0, 1.0, 1.0, 1.0),
                    border_radius: Dimension::Percent(0.5),
                    ..Default::default()
                },
                FlexboxLayout {
                    position_type: PositionType::Absolute,
                    position: Rect::<Dimension>::from_percent(self.value / self.max_value, 0.0, 0.0, 0.0),
                    margin: Rect::<Dimension>::from_points(-8.0, 0.0, -7.0, 0.0),
                    size: Size {
                        width: Dimension::Points(16.0),
                        height: Dimension::Points(16.0),
                    },
                    ..Default::default()
                },
            ),
        ])])
    }
}
