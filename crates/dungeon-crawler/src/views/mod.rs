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
use self::{main_menu::MainMenu, transition::Transition};
use ui::{widgets::*, Event, MouseData};
mod settings;
mod style;
mod transition;

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    state: ui::State,
    view: Transition<GameState>,
    element_rects: Vec<NodeLayout>,
    main_menu: MainMenu,
    mouse_key: Option<(String, ui::widgets::NodeLayout)>,
}

impl Views {
    pub fn new(ctx: &mut engine::Context, scale: f32, view: GameState) -> Self {
        ImageContext::add_texture(ctx, "logo", engine::file::read_bytes("icon.png"));
        ImageContext::add_texture(ctx, "menu", engine::file::read_bytes("icons/menu.png"));
        ImageContext::add_texture(ctx, "health", engine::file::read_bytes("icons/health.png"));
        ImageContext::add_texture(ctx, "attack", engine::file::read_bytes("icons/attack.png"));
        ImageContext::add_texture(ctx, "check", engine::file::read_bytes("icons/check.png"));

        Self {
            ui_scale: 1000.0 / scale / ctx.settings.ui_scale,
            ui: ui::Ui::new(),
            state: ui::State::new(),
            view: Transition::new(view),
            element_rects: vec![],
            main_menu: MainMenu::new(),
            mouse_key: None,
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, world: &mut World, frame_time: f32) {
        self.view.set(world.game_state.clone());
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let opacity = self.view.tick();

        let mut root = match self.view.state {
            world::GameState::Reload | world::GameState::Loading => splash::splash(),
            world::GameState::Running => game::game(ctx, &mut self.state, world),
            world::GameState::MainMenu => self.main_menu.draw(ctx, &mut self.state, world),
            world::GameState::Terminated => unreachable!(),
        };

        let mouse = { &world.components.get_resource::<resources::Input>().unwrap().mouse };
        let nodes = self.ui.render(ctx, &mut root, ui_scale_x, self.ui_scale);
        let sx = ctx.viewport.width as f32 / ui_scale_x;
        let sy = ctx.viewport.height as f32 / self.ui_scale;
        let mouse_pos = Point2::new(mouse.position.x / sx, mouse.position.y / sy);

        self.element_rects.clear();

        if let Some((mouse_key, layout)) = &self.mouse_key {
            self.state.set_event(
                &Some(mouse_key.clone()),
                Event::MouseDown(MouseData {
                    x: (mouse_pos.x - layout.x) / layout.width,
                    y: (mouse_pos.y - layout.y) / layout.height,
                }),
            );
        }

        for (layout, widget) in nodes {
            let position = Point2::new(layout.x * sx, layout.y * sy);
            let size = Point2::new(layout.width * sx, layout.height * sy);

            match widget {
                RenderWidget::Text(data) => {
                    GlyphPipeline::queue(
                        ctx,
                        GlyphProps {
                            position,
                            text: data.text.clone(),
                            size: data.size * sy,
                            color: Vector4::new(1.0, 1.0, 1.0, opacity),
                            ..Default::default()
                        },
                    );
                }
                RenderWidget::Asset(data) => {
                    self.element_rects.push(layout.clone());

                    let background = if is_hover(&mouse_pos, &layout) {
                        match mouse.state {
                            input::PressState::Released(repeat) => {
                                self.mouse_key = None;

                                if !repeat {
                                    self.state.set_event(
                                        &data.key,
                                        Event::Click(MouseData {
                                            x: mouse_pos.x,
                                            y: mouse_pos.y,
                                        }),
                                    );
                                }

                                if mouse.touch {
                                    data.background
                                } else {
                                    data.background_hover.unwrap_or(data.background)
                                }
                            }
                            input::PressState::Pressed(_) => {
                                if self.mouse_key.is_none() {
                                    if let Some(key) = &data.key {
                                        self.mouse_key = Some((key.clone(), layout.clone()));
                                    }
                                }
                                data.background_pressed.unwrap_or(data.background)
                            }
                        }
                    } else {
                        data.background
                    };

                    if data.visible {
                        let (background_end, gradient_angle) = if let Some(gradient) = &data.gradient {
                            (gradient.background_end, gradient.angle)
                        } else {
                            (background, 0.0)
                        };

                        ctx.images.queue(
                            context::Data {
                                position,
                                size,
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
                }
                _ => {}
            }
        }
    }

    pub fn to_ui_coords(&self, ctx: &engine::Context, coords: &Point2<f32>) -> Point2<f32> {
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let sx = ctx.viewport.width as f32 / ui_scale_x;
        let sy = ctx.viewport.height as f32 / self.ui_scale;

        Point2::new(coords.x / sx, coords.y / sy)
    }

    pub fn within_ui(&self, mp: &Point2<f32>) -> bool {
        for rect in self.element_rects.iter() {
            if is_hover(mp, &rect) {
                return true;
            }
        }

        false
    }
}

fn is_hover(mp: &Point2<f32>, layout: &NodeLayout) -> bool {
    mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height
}
