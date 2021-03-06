use super::{base, NodeLayout, RenderWidget};
use taffy::prelude::*;

pub struct NodeWidget {
    layout: FlexboxLayout,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
}

impl NodeWidget {
    pub fn new(layout: FlexboxLayout) -> Box<Self> {
        Box::new(Self {
            layout,
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
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(ctx, taffy)).collect();
        let node = taffy.new_with_children(self.layout, &children).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        self.children.iter().map(|c| c.get_nodes(taffy, &layout)).flat_map(|c| c).collect()
    }
}
