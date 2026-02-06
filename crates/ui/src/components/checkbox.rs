use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Checkbox {
    pub key: String,
    pub checked: bool,
}

impl Checkbox {
    pub fn draw(&self) -> Box<DisplayWidget> {
        DisplayWidget::new(
            DisplayWidgetProps {
                background: Vector4::new(0.0, 0.0, 0.0, 0.5),
                background_hover: Some(Vector4::new(0.3, 0.3, 0.3, 0.5)),
                background_pressed: Some(Vector4::new(0.5, 0.5, 0.5, 0.5)),
                shadow_color: Vector4::new(1.0, 1.0, 1.0, 1.0),
                shadow_radius: length(2.0),
                border_radius: length(4.0),
                ..Default::default()
            },
            Style {
                padding: Rect {
                    left: length(4.0),
                    right: length(4.0),
                    top: length(4.0),
                    bottom: length(4.0),
                },
                ..Default::default()
            },
        )
        .with_key(self.key.as_str())
        .with_children(vec![DisplayWidget::new(
            DisplayWidgetProps {
                asset_id: Some("check".into()),
                visible: self.checked,
                ..Default::default()
            },
            Style {
                size: Size {
                    width: length(32.0),
                    height: length(32.0),
                },
                ..Default::default()
            },
        )])
    }
}
