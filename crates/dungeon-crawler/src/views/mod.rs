use crate::world::resources::{self, input};
use cgmath::*;
use engine::pipelines::{
    image::{self, context::ImageContext},
    GlyphPipeline,
};
use std::collections::HashMap;
use ui::{components::ButtonComponent, prelude::*, widgets::*};

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    asset_transitions: HashMap<String, Vector4<f32>>,
}

impl Views {
    pub fn new(ctx: &mut engine::Context) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("/icon.png"));

        Self {
            ui_scale: 1000.0,
            ui: ui::Ui::new(),
            asset_transitions: HashMap::new(),
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, input: &input::Input, components: &bevy_ecs::world::World) -> bool {
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
                        background: Vector4::new(0.0, 0.0, 0.0, 0.8),
                        ..Default::default()
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
                    "button",
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
        let mut blocking = false;

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
                    let bg = data.background;
                    let prev = if let Some(key) = &data.key {
                        *self.asset_transitions.get(key).unwrap_or(&bg)
                    } else {
                        bg
                    };

                    let background = if is_hover(input.mouse.position, &layout, sx, sy) {
                        blocking = true;
                        if input.mouse.pressed {
                            prev.lerp(data.background_pressed.unwrap_or(bg), 0.05)
                        } else {
                            prev.lerp(data.background_hover.unwrap_or(bg), 0.05)
                        }
                    } else {
                        prev.lerp(bg, 0.05)
                    };

                    if let Some(key) = data.key {
                        if prev != background {
                            *self.asset_transitions.entry(key).or_insert(bg) = background;
                        }
                    }

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

        blocking
    }
}

fn is_hover(mp: Point2<f32>, layout: &NodeLayout, sx: f32, sy: f32) -> bool {
    let mp = Point2::new(mp.x / sx, mp.y / sy);
    mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height
}
