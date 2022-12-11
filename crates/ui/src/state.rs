use cgmath::*;
use engine::{
    config,
    ecs::resources::input::mouse::{self},
    utils,
};
use fxhash::{FxHashMap, FxHashSet};

use crate::widgets::NodeLayout;

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

#[derive(Debug, Clone)]
pub enum WidgetState {
    None,
    Hover,
    Pressed,
    Clicked,
}

#[derive(Default)]
pub struct State {
    pub blocked: bool,
    transitions: FxHashMap<String, Vector4<f32>>,
    pub events: FxHashMap<String, Event>,
    pub locks: FxHashSet<u64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            blocked: false,
            transitions: FxHashMap::default(),
            events: FxHashMap::default(),
            locks: FxHashSet::default(),
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

        if let Some(Event::Click(data)) = self.events.get(key) {
            let data = *data;
            self.events.remove(key);
            utils::vibrate(config::VIBRATION_LENGTH);
            return Some(data);
        }

        None
    }

    pub fn mouse_down(&mut self, key: &str) -> Option<MouseData> {
        if let Some(Event::MouseDown(data)) = self.events.get(key) {
            let data = *data;
            self.events.remove(key);
            return Some(data);
        }

        None
    }

    pub fn process(
        &mut self,
        key: &Option<String>,
        layout: &NodeLayout,
        input: &engine::ecs::resources::Input,
        scale: Point2<f32>,
    ) -> WidgetState {
        let mut state = WidgetState::None;

        if key.is_none() {
            return state;
        }

        for (id, button) in input.mouse.iter() {
            if let Some(press_position) = on_element(&button.press_position, layout, scale) {
                match button.state {
                    mouse::PressState::Released(repeat) => {
                        self.locks.remove(id);

                        if !repeat {
                            state = WidgetState::Clicked;
                            self.set_event(
                                &key,
                                Event::Click(MouseData {
                                    x: (press_position.x - layout.x) / layout.width,
                                    y: (press_position.y - layout.y) / layout.height,
                                }),
                            );
                        }
                    }
                    mouse::PressState::Pressed(_) => {
                        self.locks.insert(*id);
                        state = WidgetState::Pressed;

                        if let Some(position) = button.position {
                            let position = to_relative(&position, scale);
                            self.set_event(
                                &key,
                                Event::MouseDown(MouseData {
                                    x: (position.x - layout.x) / layout.width,
                                    y: (position.y - layout.y) / layout.height,
                                }),
                            );
                        }
                    }
                }

                break;
            } else if on_element(&button.position, layout, scale).is_some() {
                state = WidgetState::Hover;
                break;
            } else {
                state = WidgetState::None;
            }
        }

        state
    }
}

fn to_relative(mp: &Point2<f32>, scale: Point2<f32>) -> Point2<f32> {
    point2(mp.x / scale.x, mp.y / scale.y)
}

fn on_element(mp: &Option<Point2<f32>>, layout: &NodeLayout, scale: Point2<f32>) -> Option<Point2<f32>> {
    if let Some(mp) = mp {
        let mp = to_relative(mp, scale);
        if mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height {
            Some(mp)
        } else {
            None
        }
    } else {
        None
    }
}
