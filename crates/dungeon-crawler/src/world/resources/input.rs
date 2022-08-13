use cgmath::*;
use std::collections::HashMap;
use winit::event::VirtualKeyCode;

use crate::config;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum PressState {
    Released(bool),
    Pressed(bool),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiActionCode {
    Attack,
    Health,
}

#[derive(Debug)]
pub struct Joystick {
    pub id: u64,
    pub touch: bool,
    pub center: Option<Point2<f32>>,
    pub current: Option<Point2<f32>>,
    pub strength: f32,
}

#[derive(Debug)]
pub struct Mouse {
    pub id: u64,
    pub position: Point2<f32>,
    pub relative: Point2<f32>,
    pub state: PressState,
    pub touch: bool,
}

impl Mouse {
    pub fn is_pressed(&self) -> bool {
        match self.state {
            PressState::Released(_) => false,
            PressState::Pressed(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    pub keys: HashMap<VirtualKeyCode, PressState>,
    pub ui: HashMap<UiActionCode, PressState>,
    pub mouse: Mouse,
    pub joystick: Option<Joystick>,
    pub blocked: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: HashMap::new(),
            ui: HashMap::new(),
            mouse: Mouse {
                id: 0,
                position: Point2::new(0.0, 0.0),
                relative: Point2::new(0.0, 0.0),
                state: PressState::Released(true),
                touch: false,
            },
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

    pub fn mouse_move(&mut self, id: u64, position: Point2<f32>, width: u32, height: u32) {
        if let Some(joystick) = &mut self.joystick {
            let relative = Point2::new(position.x / width as f32 * 2.0 - 1.0, position.y / height as f32 * 2.0 - 1.0);

            if joystick.id == id {
                if let Some(center) = joystick.center {
                    joystick.strength = (relative.distance(center) * config::JOYSTICK_SENSITIVITY).min(1.0);
                    let angle = (relative.y - center.y).atan2(relative.x - center.x);

                    let x = joystick.strength * angle.cos();
                    let y = joystick.strength * angle.sin();
                    joystick.current = Some(Point2::new(x, y));
                } else {
                    joystick.center = Some(if joystick.touch { relative } else { Point2::new(0.0, 0.0) });
                }
            }
        }

        self.mouse.position = position;
        self.mouse.relative = Point2::new(position.x / width as f32 * 2.0 - 1.0, position.y / height as f32 * 2.0 - 1.0);
    }

    pub fn update(&mut self) {
        match self.mouse.state {
            PressState::Released(_) => {
                self.mouse.state = PressState::Released(true);
            }
            PressState::Pressed(_) => {
                self.mouse.state = PressState::Pressed(true);
            }
        }
    }

    pub fn mouse_set_pressed(&mut self, id: u64, touch: bool, pressed: bool, on_ui: bool) {
        self.mouse.touch = touch;
        match pressed {
            true => self.mouse.state = PressState::Pressed(false),
            false => self.mouse.state = PressState::Released(false),
        };

        if on_ui {
            return;
        }

        if pressed {
            if self.joystick.is_none() {
                self.joystick = Some(Joystick {
                    id,
                    touch,
                    strength: 0.0,
                    center: None,
                    current: None,
                });
            }
        } else {
            if let Some(joystick) = &mut self.joystick {
                if joystick.id == id {
                    self.joystick = None;
                    return;
                }
            }
        }
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
}
