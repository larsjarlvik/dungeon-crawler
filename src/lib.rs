use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod config;
mod state;
mod viewport;
mod world;

fn render(state: &mut state::State, start_time: &Instant, control_flow: &mut ControlFlow) {
    state.update(start_time.elapsed().as_millis() as u64);
    match state.render() {
        Ok(_) => {}
        Err(wgpu::SwapChainError::Lost) => state.resize(state.viewport.clone()),
        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        Err(e) => eprintln!("{:?}", e),
    }
}

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "info", tag = "dungeon-crawler"))
)]
pub fn main() {
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

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(state) = &mut state {
                render(state, &start_time, control_flow);
            }
        }

        match event {
            #[cfg(not(target_os = "android"))]
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                if let Some(state) = &mut state {
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            state.resize(viewport::Viewport::new(
                                physical_size.width,
                                physical_size.height,
                                window.scale_factor(),
                            ));
                        }
                        WindowEvent::ScaleFactorChanged {
                            new_inner_size,
                            scale_factor,
                        } => {
                            state.resize(viewport::Viewport::new(
                                new_inner_size.width,
                                new_inner_size.height,
                                *scale_factor,
                            ));
                        }
                        _ => {}
                    }
                }
            }
            Event::Resumed => {
                state = Some(pollster::block_on(state::State::new(&window)));
            }
            Event::Suspended => {
                state = None;
            }
            _ => {}
        };
    });
}
