use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Scroll {
    key: String,
    position: f32,
}

pub struct ScrollProps {
    pub padding: Rect<Dimension>,
}

impl Default for ScrollProps {
    fn default() -> Self {
        Self {
            padding: Default::default(),
        }
    }
}

impl Scroll {
    pub fn new(key: &str, position: f32) -> Self {
        Self { key: key.into(), position }
    }

    pub fn draw(&self, props: ScrollProps, children: Vec<Box<dyn BaseWidget>>) -> Box<NodeWidget> {
        let handle_color = vec4(1.0, 1.0, 1.0, 0.7);
        let handle_width = 20.0;
        let handle_padding = 7.0;

        NodeWidget::new(Style {
            flex_grow: 1.0,
            ..Default::default()
        })
        .with_children(vec![
            DisplayWidget::new(
                DisplayWidgetProps {
                    overflow: false,
                    background: vec4(0.0, 0.0, 0.0, 0.2),
                    ..Default::default()
                },
                Style {
                    flex_direction: FlexDirection::Row,
                    flex_grow: 1.0,
                    size: Size {
                        width: Dimension::Auto,
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                },
            )
            .with_children(vec![DisplayWidget::new(
                DisplayWidgetProps {
                    offset: vec2(0.0, -self.position),
                    locked_offset: true,
                    ..Default::default()
                },
                Style {
                    padding: props.padding,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    flex_shrink: 0.0,
                    ..Default::default()
                },
            )
            .with_children(children)]),
            DisplayWidget::new(
                DisplayWidgetProps {
                    background: vec4(0.0, 0.0, 0.0, 0.3),
                    ..Default::default()
                },
                Style {
                    size: Size {
                        width: Dimension::Points(handle_width),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                },
            )
            .with_children(vec![DisplayWidget::new(
                DisplayWidgetProps {
                    background: handle_color,
                    background_hover: Some(handle_color.lerp(vec4(1.0, 1.0, 1.0, 1.0), 0.5)),
                    background_pressed: Some(handle_color.lerp(vec4(1.0, 1.0, 1.0, 1.0), 1.0)),
                    border_radius: Dimension::Points(8.0),
                    offset: vec2(0.0, self.position),
                    ..Default::default()
                },
                Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        width: Dimension::Percent(1.0),
                        height: Dimension::Percent(0.3),
                    },
                    ..Default::default()
                },
            )
            .with_key(format!("{}_handle", self.key).as_str())]),
            NodeWidget::new(Style {
                flex_direction: FlexDirection::RowReverse,
                size: Size {
                    width: Dimension::Points(1.0),
                    height: Dimension::Percent(1.0),
                },
                ..Default::default()
            })
            .with_children(vec![NodeWidget::new(Style {
                position_type: PositionType::Absolute,
                position: Rect::<Dimension>::from_points(-(handle_width * handle_padding + handle_width) / 2.0, 0.0, 0.0, 0.0),
                size: Size {
                    width: Dimension::Points(handle_width * handle_padding),
                    height: Dimension::Percent(1.0),
                },
                ..Default::default()
            })
            .with_key(self.key.as_str())]),
        ])
    }
}
