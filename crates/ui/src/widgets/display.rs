use super::{
    base::{self},
    Gradient, NodeLayout, RenderParams,
};
use crate::WidgetState;
use cgmath::*;
use engine::pipelines::ui_element::context::{self, ImageContext};
use taffy::prelude::*;

#[derive(Debug)]
pub struct DisplayWidgetProps {
    pub asset_id: Option<String>,
    pub background: Vector4<f32>,
    pub gradient: Option<Gradient>,
    pub foreground: Vector4<f32>,
    pub background_hover: Option<Vector4<f32>>,
    pub background_pressed: Option<Vector4<f32>>,
    pub border_radius: Dimension,
    pub shadow_radius: Dimension,
    pub shadow_offset: Option<Vector2<f32>>,
    pub shadow_color: Vector4<f32>,
    pub overflow: bool,
    pub visible: bool,
    pub offset: Vector2<f32>,
    pub locked_offset: bool,
}

impl Default for DisplayWidgetProps {
    fn default() -> Self {
        Self {
            asset_id: None,
            background: Vector4::new(0.0, 0.0, 0.0, 0.0),
            foreground: Vector4::new(0.0, 0.0, 0.0, 0.0),
            background_hover: None,
            background_pressed: None,
            border_radius: Dimension::default(),
            shadow_radius: Dimension::default(),
            shadow_offset: None,
            gradient: None,
            shadow_color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            overflow: true,
            visible: true,
            offset: Vector2::new(0.0, 0.0),
            locked_offset: false,
        }
    }
}

pub struct DisplayWidget {
    pub key: Option<String>,
    pub data: DisplayWidgetProps,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    node: Option<Node>,
    style: Style,
}

impl DisplayWidget {
    pub fn new(data: DisplayWidgetProps, style: Style) -> Box<Self> {
        Box::new(Self {
            key: None,
            data,
            style,
            node: None,
            children: vec![],
        })
    }

    pub fn with_key(mut self, key: &str) -> Box<Self> {
        self.key = Some(key.into());
        Box::new(self)
    }

    pub fn with_children(mut self, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        self.children = children;
        Box::new(self)
    }
}

impl base::BaseWidget for DisplayWidget {
    fn calculate_layout(&mut self, engine: &mut engine::Engine, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.calculate_layout(engine, taffy)).collect();
        let node = if children.len() > 0 {
            taffy.new_with_children(self.style, &children).unwrap()
        } else {
            taffy.new_leaf(self.style).unwrap()
        };
        self.node = Some(node);
        node
    }

    fn render<'a>(
        &self,
        taffy: &Taffy,
        engine: &mut engine::Engine,
        input: &mut engine::ecs::resources::Input,
        state: &mut crate::state::State,
        parent_layout: &NodeLayout,
        params: &mut RenderParams,
    ) {
        let layout = taffy.layout(self.node.unwrap()).expect("Failed to layout node!");
        let mut layout = NodeLayout::new(parent_layout, layout);

        if parent_layout.width < layout.width {
            layout.x += self.data.offset.x * (parent_layout.width - layout.width).abs();
        }
        if parent_layout.height < layout.height || !self.data.locked_offset {
            layout.y += self.data.offset.y * (parent_layout.height - layout.height).abs();
        }

        let position = Point2::new(layout.x * params.scale.x, layout.y * params.scale.y);
        let size = Point2::new(layout.width * params.scale.x, layout.height * params.scale.y);

        let widget_state = state.process(&self.key, &layout, input, params.scale);

        let background = match widget_state {
            WidgetState::None => self.data.background,
            WidgetState::Hover | WidgetState::Clicked => self.data.background_hover.unwrap_or(self.data.background),
            WidgetState::Pressed => self.data.background_pressed.unwrap_or(self.data.background),
        };

        if !self.data.overflow {
            layout.clip = Some([
                (layout.x * params.scale.x) as u32,
                (layout.y * params.scale.y) as u32,
                (layout.width * params.scale.x) as u32,
                (layout.height * params.scale.y) as u32,
            ]);
        }

        if self.data.visible {
            let (background_end, gradient_angle) = if let Some(gradient) = &self.data.gradient {
                (gradient.background_end, gradient.angle)
            } else {
                (self.data.background, 0.0)
            };

            let bind_group = ImageContext::create_item(
                engine,
                context::Data {
                    position,
                    size,
                    background: state.get_transition(&self.key, background, params.frame_time),
                    background_end,
                    gradient_angle,
                    foreground: self.data.foreground,
                    border_radius: match self.data.border_radius {
                        Dimension::Points(p) => p * params.scale.y,
                        Dimension::Percent(p) => layout.height * params.scale.y * p,
                        _ => 0.0,
                    },
                    shadow_radius: match self.data.shadow_radius {
                        Dimension::Points(p) => p * params.scale.y,
                        Dimension::Percent(p) => layout.height * params.scale.y * p,
                        _ => 0.0,
                    },
                    shadow_offset: match self.data.shadow_offset {
                        Some(shadow_offset) => shadow_offset * params.scale.y,
                        None => Vector2::new(0.0, 0.0),
                    },
                    shadow_color: self.data.shadow_color,
                    opacity: params.opacity,
                    clip: layout.clip,
                },
                self.data.asset_id.clone(),
            );

            engine.ctx.images.queue(bind_group, self.data.asset_id.clone());
        }

        self.children
            .iter()
            .for_each(|c| c.render(taffy, engine, input, state, &layout, params));
    }
}
