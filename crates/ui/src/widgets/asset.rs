use super::{
    base::{self},
    AssetData, NodeLayout, RenderWidget, RenderWidgetType,
};
use taffy::prelude::*;

pub struct AssetWidget {
    pub key: Option<String>,
    pub data: AssetData,
    node: Option<Node>,
    style: Style,
}

impl AssetWidget {
    pub fn new(key: Option<String>, data: AssetData, style: Style) -> Box<Self> {
        Box::new(Self {
            key,
            data,
            style,
            node: None,
        })
    }
}

impl base::BaseWidget for AssetWidget {
    fn render(&mut self, _ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let node = taffy.new_leaf(self.style).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).expect("Failed to layout node!");
        let layout = NodeLayout::new(parent_layout, layout);

        vec![(layout, RenderWidget::new(self.key.clone(), RenderWidgetType::Asset(&self.data)))]
    }
}
