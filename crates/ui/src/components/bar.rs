use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Bar {}

pub struct BarProps {
    pub max_value: f32,
    pub value: f32,
    pub color: Vector4<f32>,
    pub width: Dimension,
}

impl Bar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, label: &str, props: BarProps) -> Box<PanelWidget> {
        PanelWidget::new(
            AssetData {
                background: Vector4::new(0.0, 0.0, 0.0, 0.8),
                ..Default::default()
            },
            FlexboxLayout {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                size: Size {
                    width: props.width,
                    height: Dimension::Auto,
                },
                padding: Rect::<Dimension>::from_points(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            vec![PanelWidget::new(
                AssetData {
                    background: props.color,
                    ..Default::default()
                },
                FlexboxLayout {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size {
                        width: Dimension::Percent(props.value / props.max_value),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                },
                vec![TextWidget::new(
                    TextData {
                        text: label.into(),
                        size: 12.0,
                    },
                    Rect::default(),
                    AlignSelf::Center,
                )],
            )],
        )
    }
}
