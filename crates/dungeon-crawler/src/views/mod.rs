use crate::world::resources::{self, input};
use cgmath::{Point2, Vector4, VectorSpace};
use engine::pipelines::{
    image::{self, context::ImageContext},
    GlyphPipeline,
};
use ui::{components::ButtonComponent, prelude::*, widgets::*};

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
}

impl Views {
    pub fn new(ctx: &mut engine::Context) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("/icon.png"));

        Self {
            ui_scale: 1000.0,
            ui: ui::Ui::new(),
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, input: &input::Input, components: &bevy_ecs::world::World) {
        let mut top_left = NodeWidget::new(
            FlexboxLayout {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            vec![],
        );

        if ctx.settings.show_fps {
            let fps = components.get_resource::<resources::Fps>().unwrap();
            top_left.children.push(TextWidget::new(
                TextData {
                    text: format!("FPS: {}", fps.fps),
                    size: 30.0,
                },
                Default::default(),
            ));
        }

        let mut root = NodeWidget::new(
            FlexboxLayout {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                padding: Rect::<Dimension>::from_points(20.0, 20.0, 20.0, 20.0),
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            vec![
                PanelWidget::new(
                    AssetData {
                        asset_id: None,
                        background: Some(Vector4::new(0.0, 0.0, 0.0, 0.8)),
                    },
                    FlexboxLayout {
                        size: Size {
                            width: Dimension::Points(300.0),
                            height: Dimension::Auto,
                        },
                        margin: Rect::<Dimension>::from_points(20.0, 20.0, 20.0, 20.0),
                        padding: Rect::<Dimension>::from_points(20.0, 20.0, 20.0, 20.0),
                        ..Default::default()
                    },
                    vec![top_left],
                ),
                ButtonComponent::new(
                    Some("logo".into()),
                    Some("This is a button".into()),
                    Rect::<Dimension>::from_points(20.0, 20.0, 20.0, 20.0),
                ),
            ],
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
                RenderWidget::Asset(data) => {
                    let bg = data.background.unwrap_or(Vector4::new(0.0, 0.0, 0.0, 0.0));
                    let background = if is_hover(input.mouse.position, &layout, sx, sy) {
                        if input.mouse.pressed {
                            bg.lerp(Vector4::new(1.0, 1.0, 1.0, 1.0), 0.2)
                        } else {
                            bg.lerp(Vector4::new(1.0, 1.0, 1.0, 1.0), 0.1)
                        }
                    } else {
                        bg
                    };

                    ctx.images.queue_image(
                        image::context::Data {
                            position: Point2::new(layout.x * sx, layout.y * sy),
                            size: Point2::new(layout.width * sx, layout.height * sy),
                            background,
                            has_image: data.asset_id.is_some(),
                        },
                        data.asset_id,
                    );
                }
                _ => {}
            }
        }
    }
}

fn is_hover(mp: Point2<f32>, layout: &NodeLayout, sx: f32, sy: f32) -> bool {
    let mp = Point2::new(mp.x / sx, mp.y / sy);
    mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height
}
