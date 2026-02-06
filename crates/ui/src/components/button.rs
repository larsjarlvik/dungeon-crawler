use crate::widgets::*;
use cgmath::*;
use taffy::prelude::*;

pub struct ButtonProps {
    pub icon: Option<(String, f32)>,
    pub text: Option<(String, f32)>,
    pub margin: Rect<LengthPercentageAuto>,
    pub padding: Rect<LengthPercentage>,
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
            margin: Rect { left: auto(), right: auto(), top: auto(), bottom: auto() },
            padding: Rect { left: length(0.0), right: length(0.0), top: length(0.0), bottom: length(0.0) },
            foreground: Vector4::new(1.0, 1.0, 1.0, 1.0),
            background: Vector4::new(0.0, 0.0, 0.0, 0.8),
            border_radius: auto(),
            shadow_radius: auto(),
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

    pub fn draw(&self, props: ButtonProps) -> Box<DisplayWidget> {
        let mut children: Vec<Box<dyn BaseWidget>> = vec![];

        if let Some((icon, size)) = props.icon.clone() {
            children.push(DisplayWidget::new(
                DisplayWidgetProps {
                    asset_id: Some(icon),
                    foreground: props.foreground,
                    ..Default::default()
                },
                Style {
                    size: Size {
                        width: length(size),
                        height: length(size),
                    },
                    ..Default::default()
                },
            ));
        }

        if let Some((text, size)) = props.text.clone() {
            children.push(TextWidget::new(
                TextData { size, text },
                Rect {
                    left: length(10.0),
                    right: length(10.0),
                    top: length(0.0),
                    bottom: length(0.0),
                },
                AlignItems::FlexStart,
            ));
        }

        DisplayWidget::new(
            DisplayWidgetProps {
                background: props.background,
                background_hover: Some(props.background.lerp(props.foreground, 0.2)),
                background_pressed: Some(props.background.lerp(props.foreground, 0.3)),
                border_radius: props.border_radius,
                shadow_radius: props.shadow_radius,
                shadow_color: props.shadow_color,
                gradient: props.gradient,
                ..Default::default()
            },
            Style {
                align_items: Some(AlignItems::Center),
                size: Size {
                    width: auto(),
                    height: auto(),
                },
                margin: props.margin,
                padding: props.padding,
                ..Default::default()
            },
        )
        .with_key(self.key.as_str())
        .with_children(children)
    }
}
