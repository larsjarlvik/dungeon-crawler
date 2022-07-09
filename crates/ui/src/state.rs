use cgmath::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Event {
    Click,
    MouseDown,
}

pub struct State {
    transitions: HashMap<String, Vector4<f32>>,
    events: HashMap<String, Event>,
}

impl State {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            events: HashMap::new(),
        }
    }

    pub fn get_transition(&mut self, key: &Option<String>, to: Vector4<f32>, frame_time: f32) -> Vector4<f32> {
        if let Some(key) = &key {
            let prev_val = *self.transitions.get(key).unwrap_or(&to);
            let new_val = prev_val.lerp(to, 10.0 * frame_time);
            *self.transitions.entry(key.clone()).or_insert(new_val) = new_val;
            return new_val;
        }

        to
    }

    pub fn set_event(&mut self, key: &Option<String>, event: Event) {
        if let Some(key) = &key {
            *self.events.entry(key.clone()).or_insert(event) = event;
        }
    }

    pub fn clicked(&mut self, key: &String) -> bool {
        if let Some(event) = self.events.get(key) {
            match event {
                Event::Click => {
                    self.events.remove(key);
                    return true;
                }
                _ => {}
            }
        }

        false
    }

    pub fn mouse_down(&mut self, key: &String) -> bool {
        if let Some(event) = self.events.get(key) {
            match event {
                Event::MouseDown => {
                    self.events.remove(key);
                    return true;
                }
                _ => {}
            }
        }

        false
    }
}
