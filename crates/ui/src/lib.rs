use taffy::prelude::*;
use widgets::*;
pub mod components;
mod state;
pub mod widgets;
pub use state::*;
pub use taffy::prelude;

#[derive(Default)]
pub struct Ui {}

impl Ui {
    pub fn render<'a>(
        &'a self,
        ctx: &mut engine::Context,
        root: &'a mut widgets::NodeWidget,
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
            .set_style(
                root_node,
                Style {
                    size: Size {
                        width: Dimension::Points(width),
                        height: Dimension::Points(height),
                    },
                    ..Default::default()
                },
            )
            .unwrap();

        taffy
            .compute_layout(
                root_node,
                Size {
                    width: AvailableSpace::Definite(width),
                    height: AvailableSpace::Definite(height),
                },
            )
            .unwrap();

        let result = {
            let nodes = root.get_nodes(&taffy, &root_layout);
            nodes.into_iter().map(|(node, widget)| (node, widget.clone())).collect()
        };

        result
    }
}
