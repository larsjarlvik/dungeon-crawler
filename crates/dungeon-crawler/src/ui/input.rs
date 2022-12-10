use cgmath::*;
use fxhash::FxHashSet;
use ui::{widgets::NodeLayout, Event, MouseData, State};

use crate::world::{
    resources::{self, input::mouse},
    World,
};

pub struct Input {
    pub locks: FxHashSet<u64>,
}

#[derive(Debug, Clone)]
pub enum WidgetState {
    None,
    Hover,
    Pressed,
    Clicked,
}

impl Input {
    pub fn new() -> Self {
        Self {
            locks: FxHashSet::default(),
        }
    }

    pub fn process(&mut self, nodes: &mut [(NodeLayout, RenderWidget)], ui_state: &mut State, world: &mut World, scale: Point2<f32>) {
        let mut input = world.components.get_resource_mut::<resources::Input>().unwrap();

        if ui_state.blocked {
            input.mouse.clear();
        }

        for (layout, widget) in nodes.iter_mut() {
            if widget.key.is_none() {
                continue;
            }

            for (id, button) in input.mouse.iter() {
                if let Some(press_position) = on_element(&button.press_position, layout, scale) {
                    match button.state {
                        mouse::PressState::Released(repeat) => {
                            self.locks.remove(id);

                            if !repeat {
                                widget.state = WidgetState::Clicked;
                                ui_state.set_event(
                                    &widget.key,
                                    Event::Click(MouseData {
                                        x: (press_position.x - layout.x) / layout.width,
                                        y: (press_position.y - layout.y) / layout.height,
                                    }),
                                );
                            }
                        }
                        mouse::PressState::Pressed(_) => {
                            self.locks.insert(*id);
                            widget.state = WidgetState::Pressed;

                            if let Some(position) = button.position {
                                let position = to_relative(&position, scale);
                                ui_state.set_event(
                                    &widget.key,
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
                    widget.state = WidgetState::Hover;
                    break;
                } else {
                    widget.state = WidgetState::None;
                }
            }
        }
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
