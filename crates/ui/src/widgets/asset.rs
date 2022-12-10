use super::{
    base::{self},
    AssetData, NodeLayout, RenderParams,
};
use crate::WidgetState;
use cgmath::*;
use engine::pipelines::ui_element::context::{self, ImageContext};
use taffy::prelude::*;

pub struct AssetWidget {
    pub key: Option<String>,
    pub data: AssetData,
    node: Option<Node>,
    style: Style,
    pub children: Vec<Box<dyn base::BaseWidget>>,
}

impl AssetWidget {
    pub fn new(key: Option<String>, data: AssetData, style: Style) -> Box<Self> {
        Box::new(Self {
            key,
            data,
            style,
            node: None,
            children: vec![],
        })
    }

    pub fn with_children(mut self, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        self.children = children;
        Box::new(self)
    }
}

impl base::BaseWidget for AssetWidget {
    fn calculate_layout(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.calculate_layout(ctx, taffy)).collect();
        let node = taffy.new_with_children(self.style, &children).unwrap();
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
        params: &RenderParams,
    ) {
        let layout = taffy.layout(self.node.unwrap()).expect("Failed to layout node!");
        let layout = NodeLayout::new(parent_layout, layout);

        let position = Point2::new(layout.x * params.scale.x, layout.y * params.scale.y);
        let size = Point2::new(layout.width * params.scale.x, layout.height * params.scale.y);

        let widget_state = state.process(&self.key, &layout, input, params.scale);
        let background = match widget_state {
            WidgetState::None => self.data.background,
            WidgetState::Hover | WidgetState::Clicked => self.data.background_hover.unwrap_or(self.data.background),
            WidgetState::Pressed => self.data.background_pressed.unwrap_or(self.data.background),
        };

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
                },
                self.data.asset_id.clone(),
            );

            engine.ctx.images.queue(bind_group, self.data.asset_id.clone());

            self.children
                .iter()
                .for_each(|c| c.render(taffy, engine, input, state, &layout, params));
        }
    }
}
