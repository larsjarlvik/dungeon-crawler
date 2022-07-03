use taffy::prelude::*;
use widgets::{BaseWidget, NodeLayout, RenderWidget};
pub mod widgets;
pub use taffy::prelude;

pub fn render(ctx: &mut engine::Context, root: &mut widgets::NodeWidget, width: f32, height: f32) -> Vec<(NodeLayout, RenderWidget)> {
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
