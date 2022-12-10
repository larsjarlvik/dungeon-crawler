use super::{base, NodeLayout, RenderParams};
use cgmath::*;
use taffy::prelude::*;

pub struct NodeWidget {
    style: Style,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
}

impl NodeWidget {
    pub fn new(style: Style) -> Box<Self> {
        Box::new(Self {
            style,
            children: vec![],
            node: None,
        })
    }

    pub fn with_children(mut self, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        self.children = children;
        Box::new(self)
    }
}

impl base::BaseWidget for NodeWidget {
    fn calculate_layout(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.calculate_layout(ctx, taffy)).collect();
        let node = taffy.new_with_children(self.style, &children).unwrap();
        self.node = Some(node);
        node
    }

    fn render(&self, taffy: &Taffy, engine: &mut engine::Engine, parent_layout: &NodeLayout, params: &RenderParams) {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        self.children.iter().for_each(|c| c.render(taffy, engine, &layout, params));
    }
}
