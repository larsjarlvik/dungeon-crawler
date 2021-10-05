use cgmath::*;
use std::collections::HashSet;
use winit::event::VirtualKeyCode;

use crate::config;

pub struct Joystick {
    pub id: u64,
    pub touch: bool,
    pub center: Option<Point2<f32>>,
    pub current: Option<Point2<f32>>,
    pub strength: f32,
}

pub struct Mouse {
    pub id: u64,
    pub position: Point2<f32>,
    pub relative: Point2<f32>,
    pub pressed: bool,
}

pub struct Input {
    pub keys: HashSet<VirtualKeyCode>,
    pub mouse: Mouse,
    pub joystick: Option<Joystick>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            mouse: Mouse {
                id: 0,
                position: Point2::new(0.0, 0.0),
                relative: Point2::new(0.0, 0.0),
                pressed: false,
            },
            joystick: None,
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

    pub fn mouse_move(&mut self, id: u64, position: Point2<f32>, width: u32, height: u32) {
        let relative = Point2::new(position.x / width as f32 * 2.0 - 1.0, position.y / height as f32 * 2.0 - 1.0);

        if let Some(joystick) = &mut self.joystick {
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
                return;
            }
        }

        self.mouse.position = position;
        self.mouse.relative = Point2::new(position.x / width as f32 * 2.0 - 1.0, position.y / height as f32 * 2.0 - 1.0);
    }

    pub fn mouse_set_pressed(&mut self, id: u64, touch: bool, pressed: bool) {
        if pressed {
            if self.joystick.is_none() {
                self.joystick = Some(Joystick {
                    id,
                    touch,
                    strength: 0.0,
                    center: None,
                    current: None,
                });
            } else {
                self.mouse.pressed = true;
            }
        } else {
            if let Some(joystick) = &mut self.joystick {
                if joystick.id == id {
                    self.joystick = None;
                    return;
                }
            }

            self.mouse.pressed = false;
        }
    }
}
