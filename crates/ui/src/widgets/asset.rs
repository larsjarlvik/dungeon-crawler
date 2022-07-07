use super::{
    base::{self, RenderWidget},
    AssetData, NodeLayout,
};
use taffy::prelude::*;

pub struct AssetWidget {
    pub data: AssetData,
    size: Size<Dimension>,
    node: Option<Node>,
    margin: Rect<Dimension>,
}

impl AssetWidget {
    pub fn new(data: AssetData, margin: Rect<Dimension>, size: Size<Dimension>) -> Box<Self> {
        Box::new(Self {
            data,
            margin,
            size,
            node: None,
        })
    }
}

impl base::BaseWidget for AssetWidget {
    fn render(&mut self, _ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let node = taffy
            .new_leaf(FlexboxLayout {
                size: self.size,
                margin: self.margin,
                ..Default::default()
            })
            .unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).expect("Failed to layout node!");
        let layout = NodeLayout::new(parent_layout, layout);

        vec![(layout, RenderWidget::Asset(self.data.clone()))]
    }
}
