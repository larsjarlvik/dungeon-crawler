use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{self, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

mod config;
mod engine;
mod state;
mod utils;
mod world;

fn render(window: &window::Window, state: &mut state::State, start_time: &Instant, control_flow: &mut ControlFlow) {
    state.update(start_time.elapsed().as_millis() as u64);
    match state.render() {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost) => state.resize(window, false),
        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        Err(e) => eprintln!("{:?}", e),
    }
}

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "info", tag = "dungeon-crawler"))
)]
pub fn main() {
    #[cfg(not(target_os = "android"))]
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Dungeon Crawler").build(&event_loop).unwrap();
    let mut input = WinitInputHelper::new();
    let start_time = Instant::now();

    #[allow(unused_assignments)]
    let mut state: Option<state::State> = None;

    #[cfg(not(target_os = "android"))]
    {
        state = Some(pollster::block_on(state::State::new(&window)));
    }

    #[cfg(target_os = "android")]
    utils::aquire_wakelock();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_released(VirtualKeyCode::R) {
                if let Some(state) = &mut state {
                    state.init();
                }
            }

            if let Some(state) = &mut state {
                render(&window, state, &start_time, control_flow);
            }
        }

        match event {
            #[cfg(not(target_os = "android"))]
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                if let Some(state) = &mut state {
                    match event {
                        WindowEvent::Resized(..) => {
                            state.resize(&window, true);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            state.resize(&window, true);
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
            _ => {}
        };
    });
}
