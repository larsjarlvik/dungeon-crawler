use std::collections::HashSet;

use cgmath::*;
use ui::{
    widgets::{NodeLayout, RenderWidget, RenderWidgetState},
    Event, MouseData, State,
};

use crate::world::{
    resources::{self, input::mouse},
    World,
};

pub struct Input {
    pub locks: HashSet<u64>,
}

impl Input {
    pub fn new() -> Self {
        Self { locks: HashSet::new() }
    }

    pub fn process(&mut self, nodes: &mut Vec<(NodeLayout, RenderWidget)>, ui_state: &mut State, world: &mut World, scale: Point2<f32>) {
        let input = &world.components.get_resource::<resources::Input>().unwrap();

        for (layout, widget) in nodes.iter_mut() {
            if widget.key.is_none() {
                continue;
            }

            for (id, button) in input.mouse.iter() {
                if let Some(press_position) = on_element(&button.press_position, &layout, scale) {
                    match button.state {
                        mouse::PressState::Released(repeat) => {
                            self.locks.remove(id);

                            if !repeat {
                                widget.state = RenderWidgetState::Clicked;
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
                            widget.state = RenderWidgetState::Pressed;

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
                } else if let Some(_) = on_element(&button.position, &layout, scale) {
                    widget.state = RenderWidgetState::Hover;
                    break;
                } else {
                    widget.state = RenderWidgetState::None;
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
