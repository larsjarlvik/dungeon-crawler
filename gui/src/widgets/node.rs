use super::{base, NodeLayout, RenderWidget};
use taffy::prelude::*;

pub struct NodeWidget {
    pub style: FlexboxLayout,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
}

impl NodeWidget {
    pub fn new(style: FlexboxLayout, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        Box::new(Self {
            style,
            children,
            node: None,
        })
    }
}

impl<'a> base::BaseWidget for NodeWidget {
    fn render(&mut self, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(taffy)).collect();

        let node = taffy.new_with_children(self.style, &children).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)> {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout {
            x: parent_layout.x + layout.location.x,
            y: parent_layout.y + layout.location.y,
            width: parent_layout.width + layout.size.width,
            height: parent_layout.height + layout.size.height,
        };

        self.children.iter().map(|c| c.get_nodes(taffy, &layout)).flat_map(|c| c).collect()
    }
}
