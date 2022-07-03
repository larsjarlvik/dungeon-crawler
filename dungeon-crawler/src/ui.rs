use gui::{
    prelude::*,
    widgets::{self, RenderWidget},
};

pub fn update(engine: &mut engine::Engine) {
    let mut root = widgets::NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        vec![
            widgets::NodeWidget::new(
                FlexboxLayout {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![widgets::TextWidget::new("Row 1 - 1"), widgets::TextWidget::new("Row 1 - 2")],
            ),
            widgets::NodeWidget::new(
                FlexboxLayout {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![widgets::TextWidget::new("Row 2 - 1"), widgets::TextWidget::new("Row 2 - 2")],
            ),
        ],
    );

    let nodes = gui::render_ui(&mut root);

    for (layout, widget) in nodes {
        match widget {
            RenderWidget::Text(text) => {
                engine
                    .glyph_pipeline
                    .queue(text, (layout.x, layout.y), (layout.width, layout.height));
            }
            _ => {}
        }
    }
}
