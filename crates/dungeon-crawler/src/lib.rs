#![allow(clippy::type_complexity)]

use crate::ui::Views;
use cgmath::Point2;
use engine::{
    ecs::resources::{input::mouse::PressState, Input},
    Settings,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use world::GameState;

mod config;
mod map;
mod state;
mod ui;
mod utils;
mod world;

#[cfg_attr(target_os = "android", ndk_glue::main(logger(level = "error", tag = "dungeon-crawler")))]
pub fn main() {
    #[cfg(not(target_os = "android"))]
    env_logger::init();

    #[cfg(target_os = "android")]
    engine::utils::aquire_wakelock();

    let settings = Settings::load();
    let mut window = WindowBuilder::new().with_title("Dungeon Crawler").with_decorations(true);

    window = if settings.fullscreen {
        window.with_fullscreen(Some(Fullscreen::Borderless(None)))
    } else {
        window
            .with_inner_size(winit::dpi::LogicalSize::new(settings.window_size[0], settings.window_size[1]))
            .with_position(winit::dpi::LogicalPosition::new(settings.window_pos[0], settings.window_pos[1]))
    };

    let event_loop = EventLoop::new();
    let window = window.build(&event_loop).expect("Failed to create window!");

    #[allow(unused_assignments)]
    let mut state: Option<state::State> = None;

    event_loop.run(move |event, _, control_flow| {
        if let Some(state) = &mut state {
            match state.world.game_state {
                GameState::Terminated => {
                    *control_flow = ControlFlow::Exit;
                }
                GameState::Reload => {
                    state.engine.ctx.settings = Settings::load();
                    state.views = Views::new(&mut state.engine, window.scale_factor() as f32);
                    state.resize(&window);
                    state.engine.reload_pipelines();
                    state.world.set_sounds(&state.engine.ctx);
                    state.world.game_state = GameState::Running;
                }
                _ => {}
            }
        }

        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                if let Some(state) = &mut state {
                    match event {
                        WindowEvent::CloseRequested => {
                            state.world.game_state = GameState::Terminated;
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(..) => {
                            state.resize(&window);
                        }
                        WindowEvent::Moved(..) => {
                            state.resize(&window);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            state.resize(&window);
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            state.keyboard(input);

                            let input = state.world.components.get_resource::<Input>().unwrap();
                            if input.is_pressed(VirtualKeyCode::Escape) {
                                *control_flow = ControlFlow::Exit;
                            }

                            if input.is_pressed(VirtualKeyCode::LControl)
                                && input.key_state(VirtualKeyCode::F) == PressState::Pressed(false)
                            {
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
                            state.mouse_press(0, false, mouse_state == &winit::event::ElementState::Pressed);
                        }
                        WindowEvent::Touch(touch) => {
                            state.mouse_move(touch.id, touch.location.x as f32, touch.location.y as f32);
                            match touch.phase {
                                TouchPhase::Started => state.mouse_press(touch.id, true, true),
                                TouchPhase::Ended | TouchPhase::Cancelled => state.mouse_press(touch.id, true, false),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::Resumed => {
                if let Some(state) = &mut state {
                    if state.engine.ctx.surface.is_none() {
                        let surface = unsafe { state.engine.ctx.instance.create_surface(&window) };
                        engine::configure_surface(
                            &surface,
                            &state.engine.ctx.device,
                            state.engine.ctx.color_format,
                            Point2::new(window.inner_size().width, window.inner_size().height),
                        );
                        state.engine.ctx.surface = Some(surface);
                    }

                    state.resize(&window);
                    state.world.reset_time();
                } else {
                    state = Some(pollster::block_on(state::State::new(&window)));
                }
            }
            Event::Suspended => {
                if let Some(state) = &mut state {
                    state.engine.ctx.surface = None;
                }
            }
            Event::MainEventsCleared => {
                if let Some(state) = &mut state {
                    if state.engine.ctx.surface.is_some() {
                        state.update();
                        state.render();
                    }
                }

                window.request_redraw();

                if let Some(state) = &mut state {
                    if state.world.resources.is_none() {
                        state.world.load_resources(&state.engine.ctx);
                        state.world.init(&mut state.engine);
                        state.world.game_state = GameState::Running;
                    }
                } else {
                    #[cfg(not(target_os = "android"))]
                    {
                        state = Some(pollster::block_on(state::State::new(&window)));
                    }
                }
            }
            _ => {}
        };
    });
}
