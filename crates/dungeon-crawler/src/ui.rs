use engine::pipelines::GlyphPipeline;
use ui::{prelude::*, widgets::*};

pub fn update(engine: &mut engine::Engine) {
    let ui_scale = 100.0;

    let mut root = NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0),
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        },
        vec![
            NodeWidget::new(
                FlexboxLayout {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                vec![
                    TextWidget::new(TextData {
                        text: "Row 1 - 1".to_string(),
                        size: 4.0,
                    }),
                    TextWidget::new(TextData {
                        text: "Row 1 - 2".to_string(),
                        size: 4.0,
                    }),
                ],
            ),
            NodeWidget::new(
                FlexboxLayout {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                vec![
                    TextWidget::new(TextData {
                        text: "Row 2 - 1".to_string(),
                        size: 4.0,
                    }),
                    TextWidget::new(TextData {
                        text: "Row 2 - 2".to_string(),
                        size: 4.0,
                    }),
                ],
            ),
        ],
    );

    let ui_scale_x = ui_scale * engine.ctx.viewport.get_aspect();
    let nodes = ui::render(&mut engine.ctx, &mut root, ui_scale_x, ui_scale);

    let sx = engine.ctx.viewport.width as f32 / ui_scale_x;
    let sy = engine.ctx.viewport.height as f32 / ui_scale;

    for (layout, widget) in nodes {
        match widget {
            RenderWidget::Text(data) => {
                GlyphPipeline::queue(
                    &mut engine.ctx,
                    data.text,
                    data.size * sy,
                    (layout.x * sx, layout.y * sy),
                    (layout.width, layout.height),
                );
            }
            _ => {}
        }
    }
}
