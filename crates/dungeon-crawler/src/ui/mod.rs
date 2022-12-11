use crate::world::{GameState, World};
use cgmath::*;
use engine::{ecs::resources::Input, pipelines::ui_element::context::ImageContext};
mod transition;
mod views;
use ui::{prelude::*, widgets::*};
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
            state: ui::State::default(),
            view: Transition::new(ViewState::Splash),
            main_menu: views::MainMenu::new(&engine.ctx),
        }
    }

    pub fn update(&mut self, engine: &mut engine::Engine, world: &mut World, frame_time: f32) {
        self.view.set(map_view_state(world));
        self.state.locks.clear();

        let ui_scale_x = self.ui_scale * engine.ctx.viewport.get_aspect();
        let opacity = self.view.tick();

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

        let mut input = world.components.get_resource_mut::<Input>().unwrap();
        self.ui.render(
            engine,
            &mut input,
            &mut self.state,
            &mut root,
            ui_scale_x,
            self.ui_scale,
            &mut RenderParams {
                scale: point2(
                    engine.ctx.viewport.width as f32 / ui_scale_x,
                    engine.ctx.viewport.height as f32 / self.ui_scale,
                ),
                opacity,
                frame_time,
                clip: [0, 0, engine.ctx.viewport.width, engine.ctx.viewport.height],
            },
        );

        self.state.blocked = opacity < 1.0;
    }

    pub fn is_click_through(&self, button_id: &u64) -> bool {
        self.state.locks.contains(button_id)
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
