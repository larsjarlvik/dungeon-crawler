use crate::widgets::*;
use cgmath::Vector4;
use taffy::prelude::*;

pub struct ButtonComponent {}

impl ButtonComponent {
    pub fn new(icon: Option<String>, text: Option<String>, margin: Rect<Dimension>) -> Box<PanelWidget> {
        let mut children: Vec<Box<dyn BaseWidget>> = vec![];

        if let Some(icon) = icon.clone() {
            children.push(AssetWidget::new(
                AssetData {
                    asset_id: Some(icon),
                    background: None,
                },
                Default::default(),
                Size {
                    width: Dimension::Points(30.0),
                    height: Dimension::Points(30.0),
                },
            ));
        }

        if let Some(text) = text.clone() {
            children.push(TextWidget::new(
                TextData { size: 20.0, text },
                Rect::<Dimension>::from_points(10.0, 10.0, 0.0, 0.0),
            ));
        }

        PanelWidget::new(
            AssetData {
                asset_id: None,
                background: Some(Vector4::new(0.0, 0.0, 0.0, 0.8)),
            },
            FlexboxLayout {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Percent(1.0),
                },
                margin,
                padding: Rect::<Dimension>::from_points(10.0, 10.0, 10.0, 10.0),
                ..Default::default()
            },
            children,
        )
    }
}
