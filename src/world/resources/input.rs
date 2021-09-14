use std::collections::HashSet;

use winit::event::VirtualKeyCode;

pub struct Input {
    pub keys: HashSet<VirtualKeyCode>,
}

impl Default for Input {
    fn default() -> Self {
        Self { keys: HashSet::new() }
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
}
