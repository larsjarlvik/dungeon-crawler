use taffy::prelude::*;
use widgets::{BaseWidget, NodeLayout, RenderWidget};
pub mod components;
mod transitions;
pub mod widgets;
pub use taffy::prelude;
pub use transitions::Transitions;

pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        ctx: &mut engine::Context,
        root: &mut Box<dyn BaseWidget>,
        width: f32,
        height: f32,
    ) -> Vec<(NodeLayout, RenderWidget)> {
        let mut taffy = Taffy::new();
        let root_node = root.render(ctx, &mut taffy);
        let root_layout = NodeLayout {
            x: 0.0,
            y: 0.0,
            width,
            height,
        };

        taffy
            .compute_layout(
                root_node,
                Size {
                    width: Some(width),
                    height: Some(height),
                },
            )
            .unwrap();

        root.get_nodes(&taffy, &root_layout)
            .iter()
            .map(|(node, widget)| (node.clone(), widget.clone()))
            .collect()
    }
}
