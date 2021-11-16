use specs::WorldExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use world::resources::input::KeyState;

mod config;
mod engine;
mod map;
mod state;
mod utils;
mod world;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "info", tag = "dungeon-crawler"))
)]
pub fn main() {
    #[cfg(not(target_os = "android"))]
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Dungeon Crawler").build(&event_loop).unwrap();

    #[allow(unused_assignments)]
    let mut state: Option<state::State> = None;

    #[cfg(not(target_os = "android"))]
    {
        state = Some(pollster::block_on(state::State::new(&window)));
    }

    #[cfg(target_os = "android")]
    utils::aquire_wakelock();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                if let Some(state) = &mut state {
                    match event {
                        WindowEvent::Resized(..) => {
                            state.resize(&window, true);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            state.resize(&window, true);
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            state.keyboard(input);

                            let input = state.world.components.read_resource::<world::resources::Input>();

                            if input.is_pressed(VirtualKeyCode::Escape) {
                                *control_flow = ControlFlow::Exit;
                            }

                            if input.is_pressed(VirtualKeyCode::LControl) && input.key_state(VirtualKeyCode::F) == KeyState::Pressed(false)
                            {
                                if window.fullscreen().is_some() {
                                    window.set_fullscreen(None);
                                } else {
                                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
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
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(&window, false),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                window.request_redraw();
            }
            _ => {}
        };
    });
}
