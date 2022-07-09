use super::{base, NodeLayout, RenderWidget};
use taffy::prelude::*;

pub struct NodeWidget {
    pub layout: FlexboxLayout,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
}

impl NodeWidget {
    pub fn new(layout: FlexboxLayout, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        Box::new(Self {
            layout,
            children,
            node: None,
        })
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
