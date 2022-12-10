use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Slider {
    pub key: String,
    pub value: f32,
    pub max_value: f32,
}

impl Slider {
    pub fn draw(&self) -> Box<NodeWidget> {
        NodeWidget::new(Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Points(48.0),
            },
            ..Default::default()
        })
        .with_key(self.key.as_str())
        .with_children(vec![DisplayWidget::new(
            DisplayWidgetProps {
                background: Vector4::new(0.0, 0.0, 0.0, 0.6),
                ..Default::default()
            },
            Style {
                size: Size {
                    width: Dimension::Points(520.0),
                    height: Dimension::Points(6.0),
                },
                ..Default::default()
            },
        )
        .with_children(vec![
            DisplayWidget::new(
                DisplayWidgetProps {
                    background: Vector4::new(1.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                Style {
                    size: Size {
                        width: Dimension::Percent(self.value / self.max_value),
                        height: Dimension::Points(6.0),
                    },
                    ..Default::default()
                },
            ),
            DisplayWidget::new(
                DisplayWidgetProps {
                    background: Vector4::new(1.0, 1.0, 1.0, 1.0),
                    border_radius: Dimension::Percent(0.5),
                    ..Default::default()
                },
                Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Dimension>::from_percent(self.value / self.max_value, 0.0, 0.0, 0.0),
                    margin: Rect::<Dimension>::from_points(-8.0, 0.0, -10.0, 0.0),
                    size: Size {
                        width: Dimension::Points(24.0),
                        height: Dimension::Points(24.0),
                    },
                    ..Default::default()
                },
            ),
        ])])
    }
}
