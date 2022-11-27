use self::{
    joystick::{Joystick, JoystickOrigin},
    mouse::{MouseButton, PressState},
};
use bevy_ecs::system::Resource;
use cgmath::*;
use fxhash::FxHashMap;
use winit::event::VirtualKeyCode;
pub mod joystick;
pub mod mouse;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiActionCode {
    Attack,
    Health,
}

#[derive(Debug, Default, Resource)]
pub struct Input {
    pub keys: FxHashMap<VirtualKeyCode, PressState>,
    pub ui: FxHashMap<UiActionCode, PressState>,
    pub mouse: FxHashMap<u64, MouseButton>,
    pub joystick: Option<Joystick>,
    pub blocked: bool,
}

impl Input {
    pub fn keyboard(&mut self, input: &winit::event::KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            match input.state {
                winit::event::ElementState::Pressed => self.keys.insert(key_code, PressState::Pressed(self.keys.contains_key(&key_code))),
                winit::event::ElementState::Released => self.keys.remove(&key_code),
            };
        }
    }

    pub fn mouse_button(&mut self, id: u64) -> &mut MouseButton {
        self.mouse.entry(id).or_insert_with(MouseButton::new)
    }

    pub fn key_state(&self, key: VirtualKeyCode) -> PressState {
        match self.keys.get(&key) {
            Some(state) => *state,
            None => PressState::Released(false),
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        match self.keys.get(&key) {
            Some(state) => match state {
                PressState::Released(_) => false,
                PressState::Pressed(_) => true,
            },
            None => false,
        }
    }

    pub fn update(&mut self) {
        if let Some(joystick) = &self.joystick {
            if let Some(button) = self.mouse.get(&joystick.id) {
                if !button.is_pressed() {
                    self.joystick = None;
                }
            }
        }

        for (_, button) in self.mouse.iter_mut() {
            match button.state {
                PressState::Released(_) => button.state = PressState::Released(true),
                PressState::Pressed(_) => button.state = PressState::Pressed(true),
            }
        }
    }

    pub fn set_from_ui(&mut self, action_code: UiActionCode, pressed: bool) {
        match pressed {
            true => self.ui.insert(action_code, PressState::Pressed(self.ui.contains_key(&action_code))),
            false => self.ui.remove(&action_code),
        };
    }

    pub fn pressed_buttons(&self) -> FxHashMap<u64, MouseButton> {
        self.mouse
            .iter()
            .filter(|(_, button)| button.is_pressed())
            .map(|(id, button)| (*id, button.clone()))
            .collect()
    }

    pub fn set_joystick(&mut self, button_id: &u64, viewport_width: u32, viewport_height: u32) {
        if let Some(mouse) = self.mouse.get(button_id) {
            if let PressState::Pressed(repeat) = mouse.state {
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
        }
    }
}
