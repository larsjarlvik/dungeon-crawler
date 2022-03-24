use engine::Settings;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use world::{resources::input::KeyState, GameState};

mod config;
mod engine;
mod map;
mod state;
mod ui;
mod utils;
mod world;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "info", tag = "dungeon-crawler"))
)]
pub fn main() {
    #[cfg(not(target_os = "android"))]
    env_logger::init();

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
    let window = window.build(&event_loop).unwrap();

    #[allow(unused_assignments)]
    let mut state: Option<state::State> = None;

    #[cfg(target_os = "android")]
    utils::aquire_wakelock();

    event_loop.run(move |event, _, control_flow| {
        if let Some(state) = &mut state {
            match state.world.game_state {
                GameState::Terminated => {
                    *control_flow = ControlFlow::Exit;
                }
                GameState::Reload => {
                    state.engine.ctx.settings = Settings::load();
                    state.resize(&window, true);
                    state.engine.reload_pipelines();
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
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(..) => {
                            state.resize(&window, true);
                        }
                        WindowEvent::Moved(..) => {
                            state.resize(&window, true);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            state.resize(&window, true);
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            state.keyboard(input);

                            let input = state.world.components.get_resource::<world::resources::Input>().unwrap();
                            if input.is_pressed(VirtualKeyCode::Escape) {
                                *control_flow = ControlFlow::Exit;
                            }

                            if input.is_pressed(VirtualKeyCode::LControl) && input.key_state(VirtualKeyCode::F) == KeyState::Pressed(false)
                            {
                                match window.fullscreen() {
                                    Some(_) => window.set_fullscreen(None),
                                    None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
                                }
                            }

                            if input.is_pressed(VirtualKeyCode::LControl) && input.key_state(VirtualKeyCode::A) == KeyState::Pressed(false)
                            {
                                state.engine.ctx.settings.smaa = !state.engine.ctx.settings.smaa;
                                state.engine.ctx.settings.store();
                                state.world.game_state = GameState::Reload;
                            }

                            if input.is_pressed(VirtualKeyCode::LControl) && input.key_state(VirtualKeyCode::S) == KeyState::Pressed(false)
                            {
                                state.engine.ctx.settings.sharpen = !state.engine.ctx.settings.sharpen;
                                state.engine.ctx.settings.store();
                                state.world.game_state = GameState::Reload;
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

                if let Some(state) = &mut state {
                    if state.is_ui_blocking() {
                        state.ui.handle_event(&event);
                    }
                }
            }
            Event::Resumed => {
                if let Some(state) = &mut state {
                    state.resize(&window, true);
                } else {
                    state = Some(pollster::block_on(state::State::new(&window)));
                }
            }
            Event::Suspended => {
                if let Some(state) = &mut state {
                    state.resize(&window, false);
                }
            }
            Event::MainEventsCleared => {
                if let Some(state) = &mut state {
                    state.update(&window);
                    match state.render(&window) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(&window, false),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                window.request_redraw();

                if let Some(state) = &mut state {
                    if state.world.resources.is_none() {
                        state.world.load_resources(&mut state.engine);
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
