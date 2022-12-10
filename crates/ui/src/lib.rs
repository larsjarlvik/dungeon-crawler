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
        engine: &mut engine::Engine,
        input: &mut engine::ecs::resources::Input,
        state: &mut crate::state::State,
        root: &'a mut widgets::NodeWidget,
        width: f32,
        height: f32,
        params: &mut RenderParams,
    ) {
        let mut taffy = Taffy::new();
        let root_node = root.calculate_layout(&mut engine.ctx, &mut taffy);
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

        root.render(&mut taffy, engine, input, state, &root_layout, params);
    }
}
