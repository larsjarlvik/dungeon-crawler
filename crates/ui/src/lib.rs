use cgmath::*;
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
        ui_scale: Point2<f32>,
        params: &mut RenderParams,
    ) {
        let mut taffy = Taffy::new();
        let root_node = root.calculate_layout(engine, &mut taffy);
        let root_layout = NodeLayout {
            x: 0.0,
            y: 0.0,
            width: ui_scale.x,
            height: ui_scale.y,
            clip: None,
        };

        taffy
            .set_style(
                root_node,
                Style {
                    size: Size {
                        width: Dimension::Points(ui_scale.x),
                        height: Dimension::Points(ui_scale.y),
                    },
                    ..Default::default()
                },
            )
            .unwrap();

        taffy
            .compute_layout(
                root_node,
                Size {
                    width: AvailableSpace::Definite(ui_scale.x),
                    height: AvailableSpace::Definite(ui_scale.y),
                },
            )
            .unwrap();

        root.render(&taffy, engine, input, state, &root_layout, params);
    }
}
