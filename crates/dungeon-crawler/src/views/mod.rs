use crate::world::resources;
use engine::pipelines::GlyphPipeline;
use ui::{prelude::*, widgets::*};

pub fn update(ctx: &mut engine::Context, components: &bevy_ecs::world::World, ui: &ui::Ui) {
    let ui_scale = 100.0;

    let mut top_left = NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        vec![],
    );

    if ctx.settings.show_fps {
        let fps = components.get_resource::<resources::Fps>().unwrap();
        top_left.children.push(TextWidget::new(TextData {
            text: format!("FPS: {}", fps.fps),
            size: 4.0,
        }));
    }

    let mut root = NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: Rect::<Dimension>::from_points(2.0, 2.0, 2.0, 2.0),
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        },
        vec![top_left],
    );

    let ui_scale_x = ui_scale * ctx.viewport.get_aspect();
    let nodes = ui.render(ctx, &mut root, ui_scale_x, ui_scale);

    let sx = ctx.viewport.width as f32 / ui_scale_x;
    let sy = ctx.viewport.height as f32 / ui_scale;

    for (layout, widget) in nodes {
        match widget {
            RenderWidget::Text(data) => {
                GlyphPipeline::queue(
                    ctx,
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
