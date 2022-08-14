use crate::world::{self, GameState, World};
use cgmath::*;
use engine::pipelines::{
    glyph::*,
    ui_element::context::{self, ImageContext},
    GlyphPipeline,
};
mod transition;
mod views;
use ui::widgets::*;
mod input;
mod style;
use self::transition::Transition;

pub struct Views {
    ui_scale: f32,
    ui: ui::Ui,
    input: input::Input,
    state: ui::State,
    view: Transition<GameState>,
    main_menu: views::MainMenu,
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
            input: input::Input::new(),
            state: ui::State::new(),
            view: Transition::new(view),
            main_menu: views::MainMenu::new(&ctx),
        }
    }

    pub fn update(&mut self, ctx: &mut engine::Context, world: &mut World, frame_time: f32) {
        self.view.set(world.game_state.clone());
        let ui_scale_x = self.ui_scale * ctx.viewport.get_aspect();
        let opacity = self.view.tick();
        let scale = point2(
            ctx.viewport.width as f32 / ui_scale_x,
            ctx.viewport.height as f32 / self.ui_scale,
        );

        let mut root = match self.view.state {
            world::GameState::Reload | world::GameState::Loading => views::splash(),
            world::GameState::Running => views::game(ctx, &mut self.state, world),
            world::GameState::MainMenu => self.main_menu.draw(&mut self.state, world),
            world::GameState::Terminated => unreachable!(),
        };

        let mut nodes = self.ui.render(ctx, &mut root, ui_scale_x, self.ui_scale);
        self.input.process(&mut nodes, &mut self.state, world, scale);

        for (layout, widget) in nodes {
            let position = Point2::new(layout.x * scale.x, layout.y * scale.y);
            let size = Point2::new(layout.width * scale.x, layout.height * scale.y);

            match widget.widget {
                RenderWidgetType::Text(data) => {
                    GlyphPipeline::queue(
                        ctx,
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

                        ctx.images.queue(
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
                    }
                }
                _ => {}
            }
        }
    }

    pub fn prevent_click_through(&self, button_id: &u64) -> bool {
        self.input.locks.contains(button_id)
    }
}
