use super::{
    base::{self, RenderWidget},
    NodeLayout,
};
use taffy::prelude::*;

pub struct TextWidget {
    pub value: String,
    node: Option<Node>,
}

impl TextWidget {
    pub fn new(value: &str) -> Box<Self> {
        Box::new(Self {
            value: value.to_string(),
            node: None,
        })
    }
}

impl base::BaseWidget for TextWidget {
    fn render(&mut self, taffy: &mut Taffy) -> Node {
        let node = taffy
            .new_leaf(FlexboxLayout {
                size: Size {
                    width: Dimension::Points(500.0),
                    height: Dimension::Points(30.0),
                },
                ..Default::default()
            })
            .unwrap();
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

        vec![(layout, RenderWidget::Text(self.value.clone()))]
    }
}
