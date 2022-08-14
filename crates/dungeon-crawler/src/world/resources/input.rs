use crate::config;

use super::mouse::{MouseButton, PressState};
use cgmath::*;
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
    pub center: Option<Point2<f32>>,
    pub current: Option<Point2<f32>>,
    pub strength: f32,
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

    pub fn update(&mut self, viewport_width: u32, viewport_height: u32) {
        if let Some(mouse) = self.mouse.get(&0) {
            if mouse.is_pressed() {
                if self.joystick.is_none() {
                    self.joystick = Some(Joystick {
                        id: 0,
                        strength: 0.0,
                        center: None,
                        current: None,
                    });
                }
            } else {
                self.joystick = None;
            }
        } else {
            self.joystick = None;
        }

        if let Some(joystick) = &mut self.joystick {
            if let Some(mouse) = self.mouse.get(&joystick.id) {
                if let Some(relative) = mouse.get_relative(viewport_width, viewport_height) {
                    if let Some(center) = joystick.center {
                        joystick.strength = (relative.distance(center) * config::JOYSTICK_SENSITIVITY).min(1.0);
                        let angle = (relative.y - center.y).atan2(relative.x - center.x);

                        let x = joystick.strength * angle.cos();
                        let y = joystick.strength * angle.sin();
                        joystick.current = Some(Point2::new(x, y));
                    } else {
                        joystick.center = Some(if mouse.touch { relative } else { Point2::new(0.0, 0.0) });
                    }
                }
            }
        }
    }

    pub fn get_joystick_data(&self) -> (Option<Point2<f32>>, Option<Point2<f32>>, bool) {
        if let Some(joystick) = &self.joystick {
            if let Some(mouse) = self.mouse.get(&joystick.id) {
                return (joystick.center, joystick.current, mouse.touch);
            }
        }

        (None, None, false)
    }
}
