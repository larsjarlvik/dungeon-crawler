use super::mouse::{MouseButton, PressState};
use crate::config;
use cgmath::*;
use engine::pipelines::joystick::JoystickProperties;
use std::collections::HashMap;
use winit::event::VirtualKeyCode;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiActionCode {
    Attack,
    Health,
}

#[derive(Debug)]
pub struct Joystick {
    pub id: u64,
    pub properties: Option<JoystickProperties>,
    pub strength: f32,
}

#[derive(Debug)]
pub struct Input {
    pub keys: HashMap<VirtualKeyCode, PressState>,
    pub ui: HashMap<UiActionCode, PressState>,
    pub mouse: HashMap<u64, MouseButton>,
    pub joystick: Joystick,
    pub blocked: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: HashMap::new(),
            ui: HashMap::new(),
            mouse: HashMap::new(),
            joystick: Joystick {
                id: 0,
                strength: 0.0,
                properties: None,
            },
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

    pub fn set_joystick(&mut self, button_id: &u64, viewport_width: u32, viewport_height: u32) {
        if let Some(mouse) = self.mouse.get(&button_id) {
            if let Some(relative) = mouse.get_relative(viewport_width, viewport_height) {
                self.joystick.id = *button_id;
                self.joystick.strength = 0.0;
                self.joystick.properties = Some(JoystickProperties {
                    center: if mouse.touch { relative } else { Point2::new(0.0, 0.0) },
                    current: if mouse.touch { relative } else { Point2::new(0.0, 0.0) },
                    show_ui: mouse.touch,
                });
                dbg!("set");
            }
        }
    }

    pub fn update_joystick(&mut self, viewport_width: u32, viewport_height: u32) -> Option<JoystickProperties> {
        if let Some(properties) = &mut self.joystick.properties {
            if let Some(mouse) = self.mouse.get(&self.joystick.id) {
                if mouse.is_pressed() {
                    if let Some(relative) = mouse.get_relative(viewport_width, viewport_height) {
                        self.joystick.strength = (relative.distance(properties.center) * config::JOYSTICK_SENSITIVITY).min(1.0);
                        let angle = (relative.y - properties.center.y).atan2(relative.x - properties.center.x);

                        let x = self.joystick.strength * angle.cos();
                        let y = self.joystick.strength * angle.sin();
                        properties.current = Point2::new(x, y);
                    }
                } else {
                    self.joystick.properties = None;
                }
            }
        }

        self.joystick.properties.clone()
    }

    pub fn pressed_buttons(&self) -> HashMap<u64, MouseButton> {
        self.mouse
            .iter()
            .filter(|(_, button)| button.is_pressed())
            .map(|(id, button)| (*id, button.clone()))
            .collect()
    }
}
