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
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            size: Size {
                width: auto(),
                height: length(48.0),
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
                    width: length(520.0),
                    height: length(6.0),
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
                        width: percent(self.value / self.max_value),
                        height: length(6.0),
                    },
                    ..Default::default()
                },
            ),
            DisplayWidget::new(
                DisplayWidgetProps {
                    background: Vector4::new(1.0, 1.0, 1.0, 1.0),
                    border_radius: percent(0.5),
                    ..Default::default()
                },
                Style {
                    position: Position::Absolute,
                    inset: Rect {
                        left: percent(self.value / self.max_value),
                        right: auto(),
                        top: auto(),
                        bottom: auto(),
                    },
                    margin: Rect {
                        left: length(-8.0),
                        right: length(0.0),
                        top: length(-10.0),
                        bottom: length(0.0),
                    },
                    size: Size {
                        width: length(24.0),
                        height: length(24.0),
                    },
                    ..Default::default()
                },
            ),
        ])])
    }
}
