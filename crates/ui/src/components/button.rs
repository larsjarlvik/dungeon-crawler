use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct ButtonProps {
    pub icon: Option<(String, f32)>,
    pub text: Option<(String, f32)>,
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

pub struct Button {
    pub key: String,
}

impl Button {
    pub fn new(key: &str) -> Self {
        Self { key: key.into() }
    }

    pub fn draw(&self, props: ButtonProps) -> Box<PanelWidget> {
        let mut children: Vec<Box<dyn BaseWidget>> = vec![];

        if let Some((icon, size)) = props.icon.clone() {
            children.push(AssetWidget::new(
                AssetData {
                    asset_id: Some(icon),
                    foreground: props.foreground,
                    ..Default::default()
                },
                Default::default(),
                Size {
                    width: Dimension::Points(size),
                    height: Dimension::Points(size),
                },
            ));
        }

        if let Some((text, size)) = props.text.clone() {
            children.push(TextWidget::new(
                TextData { size, text },
                Rect::<Dimension>::from_points(10.0, 10.0, 0.0, 0.0),
            ));
        }

        PanelWidget::new(
            AssetData {
                key: Some(self.key.clone().into()),
                background: props.background,
                background_hover: Some(props.background.lerp(Vector4::new(1.0, 1.0, 1.0, 1.0), 0.2)),
                background_pressed: Some(props.background.lerp(Vector4::new(1.0, 1.0, 1.0, 1.0), 0.3)),
                ..Default::default()
            },
            FlexboxLayout {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Auto,
                },
                margin: props.margin,
                padding: Rect::<Dimension>::from_points(16.0, 16.0, 16.0, 16.0),
                ..Default::default()
            },
            children,
        )
    }
}
