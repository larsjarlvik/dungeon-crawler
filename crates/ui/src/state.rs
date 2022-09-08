use cgmath::*;
use engine::{config, utils};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct MouseData {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub enum Event {
    Click(MouseData),
    MouseDown(MouseData),
}

pub struct State {
    pub blocked: bool,
    transitions: HashMap<String, Vector4<f32>>,
    pub events: HashMap<String, Event>,
}

impl State {
    pub fn new() -> Self {
        Self {
            blocked: false,
            transitions: HashMap::new(),
            events: HashMap::new(),
        }
    }

    pub fn get_transition(&mut self, key: &Option<String>, to: Vector4<f32>, frame_time: f32) -> Vector4<f32> {
        if let Some(key) = &key {
            let prev_val = *self.transitions.get(key).unwrap_or(&to);
            let new_val = prev_val.lerp(to, 10.0 * frame_time);

            if new_val.w > 0.0 {
                *self.transitions.entry(key.clone()).or_insert(new_val) = new_val;
            }
            return new_val;
        }

        to
    }

    pub fn set_event(&mut self, key: &Option<String>, event: Event) {
        if let Some(key) = &key {
            *self.events.entry(key.clone()).or_insert(event) = event;
        }
    }

    pub fn clicked(&mut self, key: &String) -> Option<MouseData> {
        if self.blocked {
            return None;
        }

        if let Some(event) = self.events.get(key) {
            match event {
                Event::Click(data) => {
                    let data = *data;
                    self.events.remove(key);
                    utils::vibrate(config::VIBRATION_LENGTH);
                    return Some(data);
                }
                _ => {}
            }
        }

        None
    }

    pub fn mouse_down(&mut self, key: &String) -> Option<MouseData> {
        if let Some(event) = self.events.get(key) {
            match event {
                Event::MouseDown(data) => {
                    let data = *data;
                    self.events.remove(key);
                    return Some(data);
                }
                _ => {}
            }
        }

        None
    }
}
