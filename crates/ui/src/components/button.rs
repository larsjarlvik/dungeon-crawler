use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

#[derive(PartialEq)]
pub enum Variant {
    Default = 0,
    Rounded = 1,
}

pub struct ButtonProps {
    pub icon: Option<(String, f32)>,
    pub text: Option<(String, f32)>,
    pub margin: Rect<Dimension>,
    pub foreground: Vector4<f32>,
    pub background: Vector4<f32>,
    pub variant: Variant,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            icon: Default::default(),
            text: Default::default(),
            margin: Default::default(),
            foreground: Vector4::new(1.0, 1.0, 1.0, 1.0),
            background: Vector4::new(0.0, 0.0, 0.0, 0.8),
            variant: Variant::Default,
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
            let margin = if props.variant == Variant::Rounded {
                Rect::<Dimension>::from_points(8.0, 8.0, 8.0, 8.0)
            } else {
                Default::default()
            };

            children.push(AssetWidget::new(
                AssetData {
                    asset_id: Some(icon),
                    foreground: props.foreground,
                    ..Default::default()
                },
                margin,
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
                background_hover: Some(props.background.lerp(props.foreground, 0.2)),
                background_pressed: Some(props.background.lerp(props.foreground, 0.3)),
                variant: props.variant as u32,
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
