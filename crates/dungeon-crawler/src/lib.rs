#![allow(clippy::type_complexity)]

use crate::ui::Views;
use cgmath::Point2;
use engine::{
    ecs::resources::{input::mouse::PressState, Input},
    Settings,
};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, TouchPhase, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::KeyCode,
    window::{Fullscreen, Window, WindowAttributes, WindowId},
};
use world::GameState;

mod config;
mod map;
mod state;
mod ui;
mod utils;
mod world;

struct App {
    window: Option<Arc<Window>>,
    state: Option<state::State>,
    settings: Settings,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let mut attrs = WindowAttributes::default()
                .with_title("Dungeon Crawler")
                .with_decorations(true);

            attrs = if self.settings.fullscreen {
                attrs.with_fullscreen(Some(Fullscreen::Borderless(None)))
            } else {
                attrs
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        self.settings.window_size[0],
                        self.settings.window_size[1],
                    ))
                    .with_position(winit::dpi::LogicalPosition::new(
                        self.settings.window_pos[0],
                        self.settings.window_pos[1],
                    ))
            };

            let window = Arc::new(event_loop.create_window(attrs).expect("Failed to create window!"));
            self.state = Some(pollster::block_on(state::State::new(window.clone())));
            self.window = Some(window);
        } else if let Some(state) = &mut self.state {
            if state.engine.ctx.surface.is_none() {
                let window = self.window.as_ref().unwrap();
                let surface = state
                    .engine
                    .ctx
                    .instance
                    .create_surface(window.clone())
                    .expect("Failed to create surface!");
                engine::configure_surface(
                    &surface,
                    &state.engine.ctx.device,
                    state.engine.ctx.color_format,
                    Point2::new(window.inner_size().width, window.inner_size().height),
                );
                state.engine.ctx.surface = Some(surface);
            }

            let window = self.window.as_ref().unwrap();
            state.resize(window);
            state.world.reset_time();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.engine.ctx.surface = None;
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if let Some(state) = &mut self.state {
            match state.world.game_state {
                GameState::Terminated => {
                    event_loop.exit();
                    return;
                }
                GameState::Reload => {
                    let window = self.window.as_ref().unwrap();
                    state.engine.ctx.settings = Settings::load();
                    state.views = Views::new(&mut state.engine, window.scale_factor() as f32);
                    state.resize(window);
                    state.engine.reload_pipelines();
                    state.world.set_sounds(&state.engine.ctx);
                    state.world.game_state = GameState::Running;
                }
                _ => {}
            }

            match event {
                WindowEvent::CloseRequested => {
                    state.world.game_state = GameState::Terminated;
                    event_loop.exit();
                }
                WindowEvent::Resized(..) | WindowEvent::Moved(..) | WindowEvent::ScaleFactorChanged { .. } => {
                    let window = self.window.as_ref().unwrap();
                    state.resize(window);
                }
                WindowEvent::KeyboardInput { event: key_event, .. } => {
                    state.keyboard(&key_event);

                    let input = state.world.components.get_resource::<Input>().unwrap();
                    if input.is_pressed(KeyCode::Escape) {
                        event_loop.exit();
                    }

                    if input.is_pressed(KeyCode::ControlLeft)
                        && input.key_state(KeyCode::KeyF) == PressState::Pressed(false)
                    {
                        let window = self.window.as_ref().unwrap();
                        match window.fullscreen() {
                            Some(_) => window.set_fullscreen(None),
                            None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    state.mouse_move(0, position.x as f32, position.y as f32);
                }
                WindowEvent::MouseInput { state: mouse_state, .. } => {
                    state.mouse_press(0, false, mouse_state == ElementState::Pressed);
                }
                WindowEvent::Touch(touch) => {
                    state.mouse_move(touch.id, touch.location.x as f32, touch.location.y as f32);
                    match touch.phase {
                        TouchPhase::Started => state.mouse_press(touch.id, true, true),
                        TouchPhase::Ended | TouchPhase::Cancelled => state.mouse_press(touch.id, true, false),
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => {
                    if state.engine.ctx.surface.is_some() {
                        state.render();
                    }
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            if state.engine.ctx.surface.is_some() {
                state.update();
            }

            if state.world.resources.is_none() {
                state.world.load_resources(&state.engine.ctx);
                state.world.init(&mut state.engine);
                state.world.game_state = GameState::Running;
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

#[cfg_attr(target_os = "android", ndk_glue::main(logger(level = "error", tag = "dungeon-crawler")))]
pub fn main() {
    #[cfg(not(target_os = "android"))]
    env_logger::init();

    #[cfg(target_os = "android")]
    engine::utils::aquire_wakelock();

    let settings = Settings::load();
    let event_loop = EventLoop::builder().build().expect("Failed to create event loop!");
    let mut app = App {
        window: None,
        state: None,
        settings,
    };
    event_loop.run_app(&mut app).expect("Event loop error");
}
