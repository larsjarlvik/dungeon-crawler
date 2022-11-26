use super::{
    base::{self},
    AssetData, NodeLayout, RenderWidget, RenderWidgetType,
};
use taffy::prelude::*;

pub struct PanelWidget {
    pub key: Option<String>,
    pub data: AssetData,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
    style: Style,
}

impl PanelWidget {
    pub fn new(key: Option<String>, data: AssetData, style: Style) -> Box<Self> {
        Box::new(Self {
            key,
            data,
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

impl base::BaseWidget for PanelWidget {
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(ctx, taffy)).collect();
        let node = taffy.new_with_children(self.style, &children).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        let children: Vec<(NodeLayout, RenderWidget)> = self.children.iter().flat_map(|c| c.get_nodes(taffy, &layout)).collect();
        let mut nodes = vec![(layout, RenderWidget::new(self.key.clone(), RenderWidgetType::Asset(&self.data)))];
        nodes.extend(children);
        nodes
    }
}
