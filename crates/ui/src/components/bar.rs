use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

#[derive(Default)]
pub struct Bar {}

pub struct BarProps {
    pub max_value: f32,
    pub value: f32,
    pub color: Vector4<f32>,
    pub gradient: Option<Gradient>,
    pub border_color: Vector4<f32>,
    pub margin: Rect<LengthPercentageAuto>,
    pub width: Dimension,
}

impl Default for BarProps {
    fn default() -> Self {
        Self {
            max_value: Default::default(),
            value: Default::default(),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            gradient: Default::default(),
            border_color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            width: auto(),
            margin: Rect { left: auto(), right: auto(), top: auto(), bottom: auto() },
        }
    }
}

impl Bar {
    pub fn draw(&self, label: &str, props: BarProps) -> Box<DisplayWidget> {
        DisplayWidget::new(
            DisplayWidgetProps {
                background: Vector4::new(0.0, 0.0, 0.0, 0.6),
                shadow_radius: length(2.0),
                shadow_color: Vector4::new(1.0, 0.8, 0.0, 1.0),
                ..Default::default()
            },
            Style {
                flex_direction: FlexDirection::Column,
                align_items: Some(AlignItems::Stretch),
                justify_content: Some(JustifyContent::FlexStart),
                margin: props.margin,
                min_size: Size {
                    width: props.width,
                    height: auto(),
                },
                ..Default::default()
            },
        )
        .with_children(vec![
            DisplayWidget::new(
                DisplayWidgetProps {
                    background: props.color,
                    gradient: props.gradient,
                    ..Default::default()
                },
                Style {
                    position: Position::Absolute,
                    justify_content: Some(JustifyContent::Center),
                    size: Size {
                        width: percent(props.value / props.max_value),
                        height: percent(1.0),
                    },
                    ..Default::default()
                },
            ),
            TextWidget::new(
                TextData {
                    text: label.into(),
                    size: 28.0,
                },
                Rect {
                    left: length(4.0),
                    right: length(4.0),
                    top: length(2.0),
                    bottom: length(4.0),
                },
                AlignItems::Center,
            ),
        ])
    }
}
