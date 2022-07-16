use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Bar {}

pub struct BarProps {
    pub max_value: f32,
    pub value: f32,
    pub color: Vector4<f32>,
    pub gradient: Option<Gradient>,
    pub border_color: Vector4<f32>,
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
            width: Default::default(),
        }
    }
}

impl Bar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, label: &str, props: BarProps) -> Box<PanelWidget> {
        PanelWidget::new(
            AssetData {
                background: Vector4::new(0.0, 0.0, 0.0, 0.6),
                shadow_radius: Dimension::Points(1.0),
                shadow_color: Vector4::new(1.0, 0.8, 0.0, 1.0),
                ..Default::default()
            },
            FlexboxLayout {
                align_items: AlignItems::Stretch,
                size: Size {
                    width: props.width,
                    height: Dimension::Points(40.0),
                },
                ..Default::default()
            },
        )
        .with_children(vec![
            PanelWidget::new(
                AssetData {
                    background: props.color,
                    gradient: props.gradient,
                    ..Default::default()
                },
                FlexboxLayout {
                    padding: Rect::<Dimension>::from_points(2.0, 2.0, 2.0, 2.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size {
                        width: Dimension::Percent(props.value / props.max_value),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                },
            ),
            TextWidget::new(
                TextData {
                    text: label.into(),
                    size: 14.0,
                },
                Rect::default(),
                AlignSelf::Center,
            ),
        ])
    }
}
