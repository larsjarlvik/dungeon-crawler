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
        self.locks.clear();

        for (layout, widget) in nodes.iter_mut() {
            if widget.key.is_none() {
                continue;
            }

            for (id, button) in input.mouse.iter() {
                if let Some(position) = on_element(&button.press_position, &layout, scale.x, scale.y) {
                    match button.state {
                        mouse::PressState::Released(repeat) => {
                            if !repeat {
                                widget.state = RenderWidgetState::Clicked;
                                ui_state.set_event(
                                    &widget.key,
                                    Event::Click(MouseData {
                                        x: (position.x - layout.x) / layout.width,
                                        y: (position.y - layout.y) / layout.height,
                                    }),
                                );
                            }
                        }
                        mouse::PressState::Pressed(_) => {
                            widget.state = RenderWidgetState::Pressed;
                            self.locks.insert(*id);

                            ui_state.set_event(
                                &widget.key,
                                Event::MouseDown(MouseData {
                                    x: (position.x - layout.x) / layout.width,
                                    y: (position.y - layout.y) / layout.height,
                                }),
                            );
                        }
                    }
                } else if let Some(_) = on_element(&button.position, &layout, scale.x, scale.y) {
                    widget.state = RenderWidgetState::Hover;
                } else {
                    widget.state = RenderWidgetState::None;
                }
            }
        }
    }
}

fn on_element(mp: &Option<Point2<f32>>, layout: &NodeLayout, sx: f32, sy: f32) -> Option<Point2<f32>> {
    if let Some(mp) = mp {
        let mp = point2(mp.x / sx, mp.y / sy);
        if mp.x >= layout.x && mp.y >= layout.y && mp.x <= layout.x + layout.width && mp.y <= layout.y + layout.height {
            Some(mp)
        } else {
            None
        }
    } else {
        None
    }
}
