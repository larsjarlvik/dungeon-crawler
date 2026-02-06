use super::{base, NodeLayout, RenderParams};
use taffy::prelude::*;

pub struct NodeWidget {
    pub key: Option<String>,
    style: Style,
    pub children: Vec<Box<dyn base::BaseWidget>>,
    pub node: Option<NodeId>,
}

impl NodeWidget {
    pub fn new(style: Style) -> Box<Self> {
        Box::new(Self {
            style,
            children: vec![],
            node: None,
            key: None,
        })
    }

    pub fn with_key(mut self, key: &str) -> Box<Self> {
        self.key = Some(key.into());
        Box::new(self)
    }

    pub fn with_children(mut self, children: Vec<Box<dyn base::BaseWidget>>) -> Box<Self> {
        self.children = children;
        Box::new(self)
    }
}

impl base::BaseWidget for NodeWidget {
    fn calculate_layout(&mut self, engine: &mut engine::Engine, taffy: &mut TaffyTree) -> NodeId {
        let children: Vec<NodeId> = self.children.iter_mut().map(|c| c.calculate_layout(engine, taffy)).collect();
        let node = taffy.new_with_children(self.style.clone(), &children).unwrap();
        self.node = Some(node);
        node
    }

    fn render(
        &self,
        taffy: &TaffyTree,
        engine: &mut engine::Engine,
        input: &mut engine::ecs::resources::Input,
        state: &mut crate::state::State,
        parent_layout: &NodeLayout,
        params: &mut RenderParams,
    ) {
        let layout = taffy.layout(self.node.unwrap()).unwrap();
        let layout = NodeLayout::new(parent_layout, layout);

        state.process(&self.key, &layout, input, params.scale);
        self.children
            .iter()
            .for_each(|c| c.render(taffy, engine, input, state, &layout, params));
    }
}
