use super::{
    base::{self, RenderWidget},
    AssetData, NodeLayout,
};
use taffy::prelude::*;

pub struct StackWidget {
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
}

impl StackWidget {
    pub fn new(children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        Box::new(Self { children, node: None })
    }
}

impl base::BaseWidget for StackWidget {
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(ctx, taffy)).collect();
        let node = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        self.children.iter().map(|c| c.get_nodes(taffy, &layout)).flat_map(|c| c).collect()
    }
}
