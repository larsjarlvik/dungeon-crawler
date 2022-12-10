use crate::world::{GameState, World};
use cgmath::*;
use engine::pipelines::{
    glyph::*,
    ui_element::context::{self, ImageContext},
    GlyphPipeline,
};
mod transition;
mod views;
use ui::{prelude::*, widgets::*};
mod input;
mod style;
use self::transition::Transition;

#[derive(PartialEq, Clone)]
enum ViewState {
    Splash,
    InGame,
    Dead,
    MainMenu,
}

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    input: input::Input,
    state: ui::State,
    view: Transition<ViewState>,
    main_menu: views::MainMenu,
}

impl Views {
    pub fn new(engine: &mut engine::Engine, scale: f32) -> Self {
        ImageContext::add_texture(engine, "logo", engine::file::read_bytes("icon.png"));
        ImageContext::add_texture(engine, "menu", engine::file::read_bytes("icons/menu.png"));
        ImageContext::add_texture(engine, "health", engine::file::read_bytes("icons/health.png"));
        ImageContext::add_texture(engine, "attack", engine::file::read_bytes("icons/attack.png"));
        ImageContext::add_texture(engine, "check", engine::file::read_bytes("icons/check.png"));

        Self {
            ui_scale: 2000.0 / scale / engine.ctx.settings.ui_scale,
            ui: ui::Ui::default(),
            input: input::Input::new(),
            state: ui::State::default(),
            view: Transition::new(ViewState::Splash),
            main_menu: views::MainMenu::new(&engine.ctx),
        }
    }

    pub fn update(&mut self, engine: &mut engine::Engine, world: &mut World, frame_time: f32) {
        self.view.set(map_view_state(world));
        let ui_scale_x = self.ui_scale * engine.ctx.viewport.get_aspect();
        let opacity = self.view.tick();
        let scale = point2(
            engine.ctx.viewport.width as f32 / ui_scale_x,
            engine.ctx.viewport.height as f32 / self.ui_scale,
        );

        let mut root = NodeWidget::new(Style {
            size: Size {
                width: Dimension::Points(ui_scale_x),
                height: Dimension::Percent(self.ui_scale),
            },
            ..Default::default()
        })
        .with_children(vec![match self.view.state {
            ViewState::Splash => views::splash(),
            ViewState::InGame => views::game(&mut engine.ctx, &mut self.state, world),
            ViewState::Dead => views::dead(&mut self.state, world),
            ViewState::MainMenu => self.main_menu.draw(engine, &mut self.state, world),
        }]);

        let mut nodes = self.ui.render(&mut engine.ctx, &mut root, ui_scale_x, self.ui_scale);

        self.state.blocked = opacity < 1.0;
        self.input.process(&mut nodes, &mut self.state, world, scale);

        for (layout, widget) in nodes {
            let position = Point2::new(layout.x * scale.x, layout.y * scale.y);
            let size = Point2::new(layout.width * scale.x, layout.height * scale.y);

            match widget.widget {
                RenderWidgetType::Text(data) => {
                    GlyphPipeline::queue(
                        &mut engine.ctx,
                        GlyphProps {
                            position,
                            text: data.text.clone(),
                            size: data.size * scale.y,
                            color: Vector4::new(1.0, 1.0, 1.0, opacity),
                            ..Default::default()
                        },
                    );
                }
                RenderWidgetType::Asset(data) => {
                    let background = match widget.state {
                        RenderWidgetState::None => data.background,
                        RenderWidgetState::Hover | RenderWidgetState::Clicked => data.background_hover.unwrap_or(data.background),
                        RenderWidgetState::Pressed => data.background_pressed.unwrap_or(data.background),
                    };

                    if data.visible {
                        let (background_end, gradient_angle) = if let Some(gradient) = &data.gradient {
                            (gradient.background_end, gradient.angle)
                        } else {
                            (background, 0.0)
                        };

                        let bind_group = ImageContext::create_item(
                            engine,
                            context::Data {
                                position,
                                size,
                                background: self.state.get_transition(&widget.key, background, frame_time),
                                background_end,
                                gradient_angle,
                                foreground: data.foreground,
                                border_radius: match data.border_radius {
                                    ui::prelude::Dimension::Points(p) => p * scale.y,
                                    ui::prelude::Dimension::Percent(p) => layout.height * scale.y * p,
                                    _ => 0.0,
                                },
                                shadow_radius: match data.shadow_radius {
                                    ui::prelude::Dimension::Points(p) => p * scale.y,
                                    ui::prelude::Dimension::Percent(p) => layout.height * scale.y * p,
                                    _ => 0.0,
                                },
                                shadow_offset: match data.shadow_offset {
                                    Some(shadow_offset) => shadow_offset * scale.y,
                                    None => Vector2::new(0.0, 0.0),
                                },
                                shadow_color: data.shadow_color,
                                opacity,
                            },
                            data.asset_id.clone(),
                        );

                        engine.ctx.images.queue(bind_group, data.asset_id.clone());
                    }
                }
                _ => {}
            }
        }
    }

    pub fn is_click_through(&self, button_id: &u64) -> bool {
        self.input.locks.contains(button_id)
    }
}

fn map_view_state(world: &mut World) -> ViewState {
    match world.game_state {
        GameState::Reload | GameState::Terminated | GameState::Loading => ViewState::Splash,
        GameState::Running => {
            if !world.is_dead() {
                ViewState::InGame
            } else {
                ViewState::Dead
            }
        }
        GameState::MainMenu => ViewState::MainMenu,
    }
}
