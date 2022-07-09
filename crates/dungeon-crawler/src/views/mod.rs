use crate::world::{self, resources::input, GameState, World};
use cgmath::*;
use engine::pipelines::{
    glyph::*,
    image::{self, context::ImageContext},
    GlyphPipeline,
};
mod game;
mod splash;
use self::transition::Transition;
use ui::widgets::*;
mod transition;

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    transitions: ui::Transitions,
    ui_state: Transition<GameState>,
}

impl Views {
    pub fn new(ctx: &mut engine::Context, scale: f32) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("icon.png"));
        ImageContext::add_texture(ctx, "menu", engine::file::read_bytes("icons/menu.png"));

        Self {
            ui_scale: 1000.0 / scale,
            ui: ui::Ui::new(),
            transitions: ui::Transitions::new(),
            ui_state: Transition::new(GameState::Loading),
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, input: &input::Input, world: &World, frame_time: f32) -> bool {
        self.ui_state.set(world.game_state.clone());
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let opacity = self.ui_state.tick();

        let mut root = match self.ui_state.state {
            world::GameState::Reload | world::GameState::Loading => splash::splash(),
            world::GameState::Running => game::game(ctx, world),
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
                        GlyphProps {
                            position: Point2::new(layout.x + sx, layout.y + sy),
                            text: data.text.clone(),
                            size: data.size,
                            color: Vector4::new(1.0, 1.0, 1.0, opacity),
                            ..Default::default()
                        },
                    );
                }
                RenderWidget::Asset(data) => {
                    let background = if is_hover(input.mouse.position, &layout, sx, sy) {
                        blocking = true;

                        match input.mouse.state {
                            input::PressState::Released(repeat) => {
                                if !repeat {
                                    if let Some(on_click) = data.callbacks.on_click {
                                        on_click();
                                    }
                                }

                                data.background_hover.unwrap_or(data.background)
                            }
                            input::PressState::Pressed(_) => data.background_pressed.unwrap_or(data.background),
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
                            opacity,
                        },
                        data.asset_id.clone(),
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
