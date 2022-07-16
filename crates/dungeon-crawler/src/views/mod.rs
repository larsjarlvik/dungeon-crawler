use crate::world::{
    self,
    resources::{self, input},
    GameState, World,
};
use cgmath::*;
use engine::pipelines::{
    glyph::*,
    ui_element::context::{self, ImageContext},
    GlyphPipeline,
};
mod game;
mod main_menu;
mod splash;
use self::transition::Transition;
use ui::{widgets::*, Event};
mod style;
mod transition;

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    state: ui::State,
    view: Transition<GameState>,
    element_rects: Vec<NodeLayout>,
}

impl Views {
    pub fn new(ctx: &mut engine::Context, scale: f32) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("icon.png"));
        ImageContext::add_texture(ctx, "menu", engine::file::read_bytes("icons/menu.png"));
        ImageContext::add_texture(ctx, "health", engine::file::read_bytes("icons/health.png"));
        ImageContext::add_texture(ctx, "attack", engine::file::read_bytes("icons/attack.png"));

        Self {
            ui_scale: 1000.0 / scale,
            ui: ui::Ui::new(),
            state: ui::State::new(),
            view: Transition::new(GameState::Loading),
            element_rects: vec![],
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, world: &mut World, frame_time: f32) {
        self.view.set(world.game_state.clone());
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let opacity = self.view.tick();

        let mut root = match self.view.state {
            world::GameState::Reload | world::GameState::Loading => splash::splash(),
            world::GameState::Running => game::game(ctx, &mut self.state, world),
            world::GameState::MainMenu => main_menu::main_menu(&mut self.state, world),
            world::GameState::Terminated => todo!(),
        };

        let mouse = { &world.components.get_resource::<resources::Input>().unwrap().mouse };
        let nodes = self.ui.render(ctx, &mut root, ui_scale_x, self.ui_scale);
        let sx = ctx.viewport.width as f32 / ui_scale_x;
        let sy = ctx.viewport.height as f32 / self.ui_scale;
        self.element_rects.clear();

        for (layout, widget) in nodes {
            match widget {
                RenderWidget::Text(data) => {
                    GlyphPipeline::queue(
                        ctx,
                        GlyphProps {
                            position: Point2::new(layout.x * sx, layout.y * sy),
                            text: data.text.clone(),
                            size: data.size * sy,
                            color: Vector4::new(1.0, 1.0, 1.0, opacity),
                            ..Default::default()
                        },
                    );
                }
                RenderWidget::Asset(data) => {
                    self.element_rects.push(layout.clone());

                    let background = if is_hover(&mouse.position, &layout, sx, sy) {
                        match mouse.state {
                            input::PressState::Released(repeat) => {
                                if !repeat {
                                    self.state.set_event(&data.key, Event::Click);
                                }

                                if mouse.touch {
                                    data.background
                                } else {
                                    data.background_hover.unwrap_or(data.background)
                                }
                            }
                            input::PressState::Pressed(_) => {
                                self.state.set_event(&data.key, Event::MouseDown);
                                data.background_pressed.unwrap_or(data.background)
                            }
                        }
                    } else {
                        data.background
                    };

                    let (background_end, gradient_angle) = if let Some(gradient) = &data.gradient {
                        (gradient.background_end, gradient.angle)
                    } else {
                        (background, 0.0)
                    };

                    ctx.images.queue(
                        context::Data {
                            position: Point2::new(layout.x * sx, layout.y * sy),
                            size: Point2::new(layout.width * sx, layout.height * sy),
                            background: self.state.get_transition(&data.key, background, frame_time),
                            background_end,
                            gradient_angle,
                            foreground: data.foreground,
                            border_radius: match data.border_radius {
                                ui::prelude::Dimension::Points(p) => p * sy,
                                ui::prelude::Dimension::Percent(p) => layout.height * sy * p,
                                _ => 0.0,
                            },
                            shadow_radius: match data.shadow_radius {
                                ui::prelude::Dimension::Points(p) => p * sy,
                                ui::prelude::Dimension::Percent(p) => layout.height * sy * p,
                                _ => 0.0,
                            },
                            shadow_offset: match data.shadow_offset {
                                Some(shadow_offset) => shadow_offset * sy,
                                None => Vector2::new(0.0, 0.0),
                            },
                            shadow_color: data.shadow_color,
                            opacity,
                        },
                        data.asset_id.clone(),
                    );
                }
                _ => {}
            }
        }
    }

    pub fn within_ui(&self, ctx: &engine::Context, mp: &Point2<f32>) -> bool {
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let sx = ctx.viewport.width as f32 / ui_scale_x;
        let sy = ctx.viewport.height as f32 / self.ui_scale;

        for rect in self.element_rects.iter() {
            if is_hover(mp, &rect, sx, sy) {
                return true;
            }
        }

        false
    }
}

fn is_hover(mp: &Point2<f32>, layout: &NodeLayout, sx: f32, sy: f32) -> bool {
    let mp = Point2::new(mp.x / sx, mp.y / sy);
    mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height
}
