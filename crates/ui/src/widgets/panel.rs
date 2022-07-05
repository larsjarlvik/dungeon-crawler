use super::{
    base::{self, RenderWidget},
    NodeLayout,
};
use taffy::prelude::*;

#[derive(Debug, Clone)]
pub struct PanelData {
    pub background: [f32; 4],
}

pub struct PanelWidget {
    pub data: PanelData,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    node: Option<Node>,
    layout: FlexboxLayout,
}

impl PanelWidget {
    pub fn new(data: PanelData, layout: FlexboxLayout, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        Box::new(Self {
            data,
            layout,
            children,
            node: None,
        })
    }
}

impl base::BaseWidget for PanelWidget {
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(ctx, taffy)).collect();
        let node = taffy.new_with_children(self.layout, &children).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        let children: Vec<(NodeLayout, RenderWidget)> = self.children.iter().map(|c| c.get_nodes(taffy, &layout)).flat_map(|c| c).collect();
        let mut nodes = vec![(layout, RenderWidget::Panel(self.data.clone()))];
        nodes.extend(children);
        nodes
    }
}
