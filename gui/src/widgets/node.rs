use super::base;
use taffy::prelude::*;

pub struct NodeWidget {
    pub style: FlexboxLayout,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<Node>,
}

impl NodeWidget {
    pub fn new(style: FlexboxLayout, children: Vec<Box<dyn base::BaseWidget>>) -> Self {
        Self {
            style,
            children,
            node: None,
        }
    }
}

impl<'a> base::BaseWidget for NodeWidget {
    fn render(&mut self, taffy: &mut Taffy) -> Node {
        let children: Vec<Node> = self.children.iter_mut().map(|c| c.render(taffy)).collect();
        let node = taffy.new_with_children(self.style, &children).unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes(&self) -> Vec<Node> {
        self.children.iter().map(|c| c.get_nodes()).flat_map(|c| c).collect()
    }
}
