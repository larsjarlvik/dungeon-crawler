use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct ButtonProps {
    pub icon: Option<(String, f32)>,
    pub text: Option<(String, f32)>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub foreground: Vector4<f32>,
    pub background: Vector4<f32>,
    pub gradient: Option<Gradient>,
    pub border_radius: Dimension,
    pub shadow_radius: Dimension,
    pub shadow_color: Vector4<f32>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            icon: Default::default(),
            text: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            foreground: Vector4::new(1.0, 1.0, 1.0, 1.0),
            background: Vector4::new(0.0, 0.0, 0.0, 0.8),
            border_radius: Dimension::default(),
            shadow_radius: Dimension::default(),
            gradient: None,
            shadow_color: Vector4::new(0.0, 0.0, 0.0, 0.0),
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
                None,
                AssetData {
                    asset_id: Some(icon),
                    foreground: props.foreground,
                    ..Default::default()
                },
                FlexboxLayout {
                    size: Size {
                        width: Dimension::Points(size),
                        height: Dimension::Points(size),
                    },
                    ..Default::default()
                },
            ));
        }

        if let Some((text, size)) = props.text.clone() {
            children.push(TextWidget::new(
                TextData { size, text },
                Rect::<Dimension>::from_points(10.0, 10.0, 0.0, 0.0),
                AlignSelf::FlexStart,
            ));
        }

        PanelWidget::new(
            Some(self.key.clone()),
            AssetData {
                background: props.background,
                background_hover: Some(props.background.lerp(props.foreground, 0.2)),
                background_pressed: Some(props.background.lerp(props.foreground, 0.3)),
                border_radius: props.border_radius,
                shadow_radius: props.shadow_radius,
                shadow_color: props.shadow_color,
                gradient: props.gradient,
                ..Default::default()
            },
            FlexboxLayout {
                align_items: AlignItems::Center,
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Auto,
                },
                margin: props.margin,
                padding: props.padding,
                ..Default::default()
            },
        )
        .with_children(children)
    }
}
