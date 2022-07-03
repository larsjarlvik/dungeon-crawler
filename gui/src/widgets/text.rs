use super::base;
use taffy::prelude::*;

pub struct TextWidget {
    pub value: String,
    node: Option<Node>,
}

impl TextWidget {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            node: None,
        }
    }
}

impl base::BaseWidget for TextWidget {
    fn render(&mut self, taffy: &mut Taffy) -> Node {
        let node = taffy
            .new_leaf(FlexboxLayout {
                size: Size {
                    width: Dimension::Points(50.0),
                    height: Dimension::Points(10.0),
                },
                ..Default::default()
            })
            .unwrap();
        self.node = Some(node);
        node
    }

    fn get_nodes(&self) -> Vec<Node> {
        vec![self.node.unwrap()]
    }
}
