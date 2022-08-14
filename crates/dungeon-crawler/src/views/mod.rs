use crate::world::{
    self,
    resources::{self, mouse},
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
            ui_scale: 2000.0 / scale / ctx.settings.ui_scale,
            ui: ui::Ui::new(),
            state: ui::State::new(),
            view: Transition::new(view),
            element_rects: vec![],
            main_menu: MainMenu::new(&ctx),
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
            world::GameState::MainMenu => self.main_menu.draw(&mut self.state, world),
            world::GameState::Terminated => unreachable!(),
        };

        let mut nodes = self.ui.render(ctx, &mut root, ui_scale_x, self.ui_scale);
        let sx = ctx.viewport.width as f32 / ui_scale_x;
        let sy = ctx.viewport.height as f32 / self.ui_scale;

        self.element_rects.clear();

        // if let Some((mouse_key, layout)) = &self.mouse_key {
        //     self.state.set_event(
        //         &Some(mouse_key.clone()),
        //         Event::MouseDown(MouseData {
        //             x: (mouse_pos.x - layout.x) / layout.width,
        //             y: (mouse_pos.y - layout.y) / layout.height,
        //         }),
        //     );
        // }

        let input = &world.components.get_resource::<resources::Input>().unwrap();
        for (layout, widget) in nodes.iter_mut() {
            if widget.key.is_none() {
                continue;
            }

            for (id, button) in input.mouse.iter() {
                if is_hover(&button.position, &layout, sx, sy) {
                    dbg!(&widget.key);

                    match button.state {
                        mouse::PressState::Released(repeat) => {
                            if repeat {
                                widget.state = RenderWidgetState::Hover;
                            } else {
                                widget.state = RenderWidgetState::Clicked;
                            }
                        }
                        mouse::PressState::Pressed(_) => {
                            widget.state = RenderWidgetState::Pressed;
                        }
                    }
                } else {
                    widget.state = RenderWidgetState::None;
                }
            }
        }

        for (layout, widget) in nodes {
            let position = Point2::new(layout.x * sx, layout.y * sy);
            let size = Point2::new(layout.width * sx, layout.height * sy);

            match widget.widget {
                RenderWidgetType::Text(data) => {
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
                RenderWidgetType::Asset(data) => {
                    self.element_rects.push(layout.clone());

                    let background = match widget.state {
                        RenderWidgetState::None => data.background,
                        RenderWidgetState::Hover | RenderWidgetState::Clicked => data.background_hover.unwrap_or(data.background),
                        RenderWidgetState::Pressed => data.background_pressed.unwrap_or(data.background),
                    };

                    // let background = data.background;
                    // let background = if is_hover(&mouse_pos, &layout) {
                    //     match mouse.state {
                    //         mouse::PressState::Released(repeat) => {
                    //             if !repeat {
                    //                 self.state.set_event(
                    //                     &data.key,
                    //                     Event::Click(MouseData {
                    //                         x: mouse_pos.x,
                    //                         y: mouse_pos.y,
                    //                     }),
                    //                 );
                    //             }

                    //             if mouse.touch {
                    //                 data.background
                    //             } else {
                    //                 data.background_hover.unwrap_or(data.background)
                    //             }
                    //         }
                    //         mouse::PressState::Pressed(repeat) => {
                    //             if !repeat && self.mouse_key.is_none() {
                    //                 if let Some(key) = &data.key {
                    //                     self.mouse_key = Some((key.clone(), layout.clone()));
                    //                 }
                    //             }
                    //             data.background_pressed.unwrap_or(data.background)
                    //         }
                    //     }
                    // } else {
                    //     data.background
                    // };

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
                                background: self.state.get_transition(&widget.key, background, frame_time),
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

    // pub fn within_ui(&self, mp: &Point2<f32>) -> bool {
    //     for rect in self.element_rects.iter() {
    //         if is_hover(mp, &rect) {
    //             return true;
    //         }
    //     }

    //     false
    // }
}

fn is_hover(mp: &Option<Point2<f32>>, layout: &NodeLayout, sx: f32, sy: f32) -> bool {
    if let Some(mp) = mp {
        let mp = point2(mp.x / sx, mp.y / sy);
        mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height
    } else {
        false
    }
}
