use super::{
    base::{self},
    NodeLayout, RenderWidget, RenderWidgetType,
};
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
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let size = engine::pipelines::glyph::get_bounds(ctx, &self.data.text, self.data.size);

        let node = taffy
            .new_leaf(FlexboxLayout {
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

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        vec![(layout, RenderWidget::new(None, RenderWidgetType::Text(&self.data)))]
    }
}
