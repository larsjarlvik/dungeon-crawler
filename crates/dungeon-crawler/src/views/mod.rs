use crate::world::resources;
use engine::pipelines::{
    image::{self, context::ImageContext},
    GlyphPipeline,
};
use ui::{prelude::*, widgets::*};

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
}

impl Views {
    pub fn new(ctx: &mut engine::Context) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("/icon.png"));

        Self {
            ui_scale: 100.0,
            ui: ui::Ui::new(),
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, components: &bevy_ecs::world::World) {
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
                size: 3.0,
            }));
        }

        let mut root = NodeWidget::new(
            FlexboxLayout {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: Rect::<Dimension>::from_points(2.0, 2.0, 2.0, 2.0),
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            vec![PanelWidget::new(
                PanelData {
                    background: [0.0, 0.0, 0.0, 0.8],
                },
                FlexboxLayout {
                    size: Size {
                        width: Dimension::Points(30.0),
                        height: Dimension::Auto,
                    },
                    padding: Rect::<Dimension>::from_points(2.0, 2.0, 2.0, 2.0),
                    ..Default::default()
                },
                vec![
                    AssetWidget::new(
                        AssetData { id: "logo".into() },
                        Size {
                            width: Dimension::Points(4.0),
                            height: Dimension::Points(4.0),
                        },
                    ),
                    top_left,
                ],
            )],
        );

        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let nodes = self.ui.render(ctx, &mut root, ui_scale_x, self.ui_scale);
        let sx = ctx.viewport.width as f32 / ui_scale_x;
        let sy = ctx.viewport.height as f32 / self.ui_scale;

        for (layout, widget) in nodes {
            match widget {
                RenderWidget::Text(data) => {
                    GlyphPipeline::queue(
                        ctx,
                        data.text,
                        data.size * sy,
                        (layout.x * sx, layout.y * sy),
                        (f32::INFINITY, f32::INFINITY),
                    );
                }
                RenderWidget::Image(data) => {
                    ctx.images.queue_image(
                        image::context::Data {
                            position: [layout.x * sx, layout.y * sy],
                            size: [layout.width * sx, layout.height * sy],
                            background: [0.0, 0.0, 0.0, 0.0],
                            has_image: true,
                        },
                        Some(data.id),
                    );
                }
                RenderWidget::Panel(data) => {
                    ctx.images.queue_image(
                        image::context::Data {
                            position: [layout.x * sx, layout.y * sy],
                            size: [layout.width * sx, layout.height * sy],
                            background: data.background,
                            has_image: false,
                        },
                        None,
                    );
                }
                _ => {}
            }
        }
    }
}
