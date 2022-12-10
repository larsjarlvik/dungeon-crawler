use super::{
    base::{self},
    NodeLayout, RenderWidget,
};
use taffy::prelude::*;

pub struct ScrollWidget {
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub wrapper: Option<Node>,
    pub node: Option<Node>,
}

impl ScrollWidget {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            children: vec![],
            wrapper: None,
            node: None,
        })
    }

    pub fn with_children(mut self, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        self.children = children;
        Box::new(self)
    }
}

impl base::BaseWidget for ScrollWidget {
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(ctx, taffy)).collect();

        let scroll = taffy
            .new_with_children(
                Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                &children,
            )
            .unwrap();

        let wrapper = taffy
            .new_with_children(
                Style {
                    position_type: PositionType::Relative,
                    flex_grow: 1.0,
                    ..Default::default()
                },
                &vec![scroll],
            )
            .unwrap();

        self.wrapper = Some(wrapper);
        self.node = Some(scroll);
        wrapper
    }

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();

        let wrapper_layout = taffy.layout(self.wrapper.unwrap()).unwrap();
        let parent_layout = NodeLayout::new(parent_layout, wrapper_layout);
        let layout = NodeLayout::new(&parent_layout, layout);

        self.children.iter().flat_map(|c| c.get_nodes(taffy, &layout)).collect()
    }
}
