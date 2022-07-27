use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct Checkbox {
    pub key: String,
    pub checked: bool,
}

impl Checkbox {
    pub fn draw(&self) -> Box<PanelWidget> {
        PanelWidget::new(
            AssetData {
                key: self.key.clone().into(),
                background: Vector4::new(0.0, 0.0, 0.0, 0.5),
                background_hover: Some(Vector4::new(0.3, 0.3, 0.3, 0.5)),
                background_pressed: Some(Vector4::new(0.5, 0.5, 0.5, 0.5)),
                shadow_color: Vector4::new(1.0, 1.0, 1.0, 1.0),
                shadow_radius: Dimension::Points(1.0),
                border_radius: Dimension::Points(2.0),
                ..Default::default()
            },
            FlexboxLayout {
                padding: Rect::from_points(2.0, 2.0, 2.0, 2.0),
                ..Default::default()
            },
        )
        .with_children(vec![PanelWidget::new(
            AssetData {
                asset_id: Some("check".into()),
                visible: self.checked,
                ..Default::default()
            },
            FlexboxLayout {
                size: Size {
                    width: Dimension::Points(16.0),
                    height: Dimension::Points(16.0),
                },
                ..Default::default()
            },
        )])
    }
}
