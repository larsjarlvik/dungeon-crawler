use taffy::prelude::*;
use widgets::{BaseWidget, NodeLayout, RenderWidget};
pub mod widgets;
pub use taffy::prelude;

pub fn render_ui(root: &mut widgets::NodeWidget) -> Vec<(NodeLayout, RenderWidget)> {
    let mut taffy = Taffy::new();
    let root_node = root.render(&mut taffy);
    let root_layout = NodeLayout {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    taffy
        .compute_layout(
            root_node,
            Size {
                height: Some(1000.0),
                width: Some(1000.0),
            },
        )
        .unwrap();

    root.get_nodes(&taffy, &root_layout)
        .iter()
        .map(|(node, widget)| (node.clone(), widget.clone()))
        .collect()
}
