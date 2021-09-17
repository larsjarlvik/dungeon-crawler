use cgmath::*;
use std::collections::HashSet;
use winit::event::VirtualKeyCode;

pub struct Mouse {
    pub position: Point2<f32>,
    pub relative: Point2<f32>,
    pub pressed: bool,
}

pub struct Input {
    pub keys: HashSet<VirtualKeyCode>,
    pub mouse: Mouse,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            mouse: Mouse {
                position: Point2::new(0.0, 0.0),
                relative: Point2::new(0.0, 0.0),
                pressed: false,
            },
        }
    }
}

impl Input {
    pub fn keyboard(&mut self, input: &winit::event::KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            if input.state == winit::event::ElementState::Pressed {
                self.keys.insert(key_code);
            }
            if input.state == winit::event::ElementState::Released {
                self.keys.remove(&key_code);
            }
        }
    }

    pub fn mouse_move(&mut self, pos: Point2<f32>, width: u32, height: u32) {
        self.mouse.position = pos;
        self.mouse.relative = Point2::new(pos.x / width as f32 * 2.0 - 1.0, pos.y / height as f32 * 2.0 - 1.0);
    }
}
