use super::{
    base::{self},
    NodeLayout, RenderParams,
};
use cgmath::{Point2, Vector4};
use engine::pipelines::glyph::GlyphProps;
use taffy::prelude::*;

#[derive(Debug, Clone)]
pub struct TextData {
    pub text: String,
    pub size: f32,
}

pub struct TextWidget {
    pub data: TextData,
    pub margin: Rect<Dimension>,
    pub align: AlignSelf,
    node: Option<Node>,
}

impl TextWidget {
    pub fn new(data: TextData, margin: Rect<Dimension>, align: AlignSelf) -> Box<Self> {
        Box::new(Self {
            data,
            margin,
            align,
            node: None,
        })
    }
}

impl base::BaseWidget for TextWidget {
    fn calculate_layout(&mut self, engine: &mut engine::Engine, taffy: &mut Taffy) -> Node {
        let size = engine.glyph_pipeline.get_bounds(&self.data.text, self.data.size);

        let node = taffy
            .new_leaf(Style {
                size: Size {
                    width: Dimension::Points(size.width()),
                    height: Dimension::Points(size.height()),
                },
                margin: self.margin,
                align_self: self.align,
                ..Default::default()
            })
            .unwrap();
        self.node = Some(node);
        node
    }

    fn render(
        &self,
        taffy: &Taffy,
        engine: &mut engine::Engine,
        _input: &mut engine::ecs::resources::Input,
        _state: &mut crate::state::State,
        parent_layout: &NodeLayout,
        params: &mut RenderParams,
    ) {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);
        let position = Point2::new(layout.x * params.scale.x, layout.y * params.scale.y);

        engine.glyph_pipeline.queue(
            parent_layout.clip,
            GlyphProps {
                position,
                text: self.data.text.clone(),
                size: self.data.size * params.scale.y,
                color: Vector4::new(1.0, 1.0, 1.0, params.opacity),
                ..Default::default()
            },
        );
    }
}
