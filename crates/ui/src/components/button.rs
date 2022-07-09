use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct ButtonProps {
    pub icon: Option<String>,
    pub text: Option<String>,
    pub margin: Rect<Dimension>,
    pub foreground: Vector4<f32>,
    pub background: Vector4<f32>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            icon: Default::default(),
            text: Default::default(),
            margin: Default::default(),
            foreground: Vector4::new(1.0, 1.0, 1.0, 1.0),
            background: Vector4::new(0.0, 0.0, 0.0, 0.8),
        }
    }
}

pub fn button(key: &str, on_click: Callback, props: ButtonProps) -> Box<PanelWidget> {
    let mut children: Vec<Box<dyn BaseWidget>> = vec![];

    if let Some(icon) = props.icon.clone() {
        children.push(AssetWidget::new(
            AssetData {
                asset_id: Some(icon),
                foreground: Vector4::new(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            Default::default(),
            Size {
                width: Dimension::Points(24.0),
                height: Dimension::Points(24.0),
            },
        ));
    }

    if let Some(text) = props.text.clone() {
        children.push(TextWidget::new(
            TextData { size: 20.0, text },
            Rect::<Dimension>::from_points(10.0, 10.0, 0.0, 0.0),
        ));
    }

    PanelWidget::new(
        AssetData {
            key: Some(key.into()),
            background: props.background,
            background_hover: Some(props.background.lerp(Vector4::new(1.0, 1.0, 1.0, 1.0), 0.2)),
            background_pressed: Some(props.background.lerp(Vector4::new(1.0, 1.0, 1.0, 1.0), 0.3)),
            callbacks: Callbacks { on_click: Some(on_click) },
            ..Default::default()
        },
        FlexboxLayout {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Percent(1.0),
            },
            margin: props.margin,
            padding: Rect::<Dimension>::from_points(10.0, 10.0, 10.0, 10.0),
            ..Default::default()
        },
        children,
    )
}
