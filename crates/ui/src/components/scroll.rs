use crate::widgets::*;
use cgmath::vec4;
use taffy::prelude::*;

pub struct Scroll {}

impl Scroll {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, children: Vec<Box<dyn BaseWidget>>) -> Box<DisplayWidget> {
        DisplayWidget::new(
            DisplayWidgetProps {
                overflow: false,
                ..Default::default()
            },
            Style {
                position_type: PositionType::Relative,
                flex_grow: 1.0,
                ..Default::default()
            },
        )
        .with_children(vec![DisplayWidget::new(
            DisplayWidgetProps {
                background: vec4(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        )
        .with_children(children)])
    }
}
