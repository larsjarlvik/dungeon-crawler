use super::{
    joystick::JoystickOrigin,
    mouse::{MouseButton, PressState},
    Joystick,
};
use cgmath::*;
use std::collections::HashMap;
use winit::event::VirtualKeyCode;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiActionCode {
    Attack,
    Health,
}

#[derive(Debug)]
pub struct Input {
    pub keys: HashMap<VirtualKeyCode, PressState>,
    pub ui: HashMap<UiActionCode, PressState>,
    pub mouse: HashMap<u64, MouseButton>,
    pub joystick: Option<Joystick>,
    pub blocked: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: HashMap::new(),
            ui: HashMap::new(),
            mouse: HashMap::new(),
            joystick: None,
            blocked: false,
        }
    }
}

impl Input {
    pub fn keyboard(&mut self, input: &winit::event::KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            if input.state == winit::event::ElementState::Pressed {
                self.keys.insert(key_code, PressState::Pressed(self.keys.contains_key(&key_code)));
            }

            if input.state == winit::event::ElementState::Released {
                self.keys.remove(&key_code);
            }
        }
    }

    pub fn mouse_button<'a>(&'a mut self, id: u64) -> &'a mut MouseButton {
        self.mouse.entry(id).or_insert_with(|| MouseButton::new())
    }

    pub fn key_state(&self, key: VirtualKeyCode) -> PressState {
        if let Some(state) = self.keys.get(&key) {
            *state
        } else {
            PressState::Released(false)
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        if let Some(state) = self.keys.get(&key) {
            match state {
                PressState::Released(_) => false,
                PressState::Pressed(_) => true,
            }
        } else {
            false
        }
    }

    pub fn set_from_ui(&mut self, action_code: UiActionCode, pressed: bool) {
        if pressed {
            self.ui.insert(action_code, PressState::Pressed(self.ui.contains_key(&action_code)));
        } else {
            self.ui.remove(&action_code);
        }
    }

    pub fn pressed_buttons(&self) -> HashMap<u64, MouseButton> {
        self.mouse
            .iter()
            .filter(|(_, button)| button.is_pressed())
            .map(|(id, button)| (*id, button.clone()))
            .collect()
    }

    pub fn set_joystick(&mut self, button_id: &u64, viewport_width: u32, viewport_height: u32) {
        if let Some(mouse) = self.mouse.get(&button_id) {
            match mouse.state {
                PressState::Pressed(repeat) => {
                    if !repeat {
                        self.joystick = Some(Joystick {
                            id: *button_id,
                            area: point2(viewport_width as f32, viewport_height as f32),
                            origin: if mouse.touch {
                                JoystickOrigin::Relative
                            } else {
                                JoystickOrigin::Screen
                            },
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
