use super::{
    base::{self, RenderWidget},
    AssetData, NodeLayout,
};
use taffy::prelude::*;

pub struct AssetWidget {
    pub data: AssetData,
    node: Option<Node>,
    layout: FlexboxLayout,
}

impl AssetWidget {
    pub fn new(data: AssetData, layout: FlexboxLayout) -> Box<Self> {
        Box::new(Self { data, layout, node: None })
    }
}

impl base::BaseWidget for AssetWidget {
    fn render(&mut self, _ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let node = taffy.new_leaf(self.layout).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).expect("Failed to layout node!");
        let layout = NodeLayout::new(parent_layout, layout);

        vec![(layout, RenderWidget::Asset(&self.data))]
    }
}
