use crate::world::{self, resources::input, World};
use cgmath::*;
use engine::pipelines::{
    image::{self, context::ImageContext},
    GlyphPipeline,
};
mod in_game;
mod splash;
use ui::widgets::*;

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    transitions: ui::Transitions,
}

impl Views {
    pub fn new(ctx: &mut engine::Context) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("/icon.png"));
        ImageContext::add_texture(ctx, "menu", engine::file::read_bytes("/icons/menu.png"));

        Self {
            ui_scale: 1000.0,
            ui: ui::Ui::new(),
            transitions: ui::Transitions::new(),
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, input: &input::Input, world: &World, frame_time: f32) -> bool {
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();

        let mut root = match world.game_state {
            world::GameState::Reload | world::GameState::Loading => splash::splash(),
            world::GameState::Running => in_game::in_game(ctx, world),
            world::GameState::Terminated => todo!(),
        };

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
                    let background = if is_hover(input.mouse.position, &layout, sx, sy) {
                        blocking = true;
                        if input.mouse.pressed {
                            data.background_pressed.unwrap_or(data.background)
                        } else {
                            data.background_hover.unwrap_or(data.background)
                        }
                    } else {
                        data.background
                    };

                    ctx.images.queue_image(
                        image::context::Data {
                            position: Point2::new(layout.x * sx, layout.y * sy),
                            size: Point2::new(layout.width * sx, layout.height * sy),
                            background: self.transitions.get(&data.key, background, frame_time),
                            foreground: data.foreground,
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
