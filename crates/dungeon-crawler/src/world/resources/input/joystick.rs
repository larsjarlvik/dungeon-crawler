use super::mouse;
use crate::config;
use cgmath::*;
use engine::pipelines::joystick::JoystickProperties;
use fxhash::FxHashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum JoystickOrigin {
    Screen,
    Relative,
}

#[derive(Debug)]
pub struct Joystick {
    pub id: u64,
    pub origin: JoystickOrigin,
    pub area: Point2<f32>,
}

impl Joystick {
    pub fn get_properties(&self, mouse: &FxHashMap<u64, mouse::MouseButton>) -> Option<JoystickProperties> {
        if let Some((center, direction, _)) = self.get_center_direction_strength(mouse) {
            Some(JoystickProperties {
                center,
                current: Point2::new(direction.x, direction.y),
                show_ui: self.origin == JoystickOrigin::Relative,
            })
        } else {
            None
        }
    }

    pub fn get_direction_strength(&self, mouse: &FxHashMap<u64, mouse::MouseButton>) -> Option<(Vector2<f32>, f32)> {
        if let Some((_, direction, strength)) = self.get_center_direction_strength(mouse) {
            Some((direction, strength))
        } else {
            None
        }
    }

    fn get_center_direction_strength(&self, mouse: &FxHashMap<u64, mouse::MouseButton>) -> Option<(Point2<f32>, Vector2<f32>, f32)> {
        if let Some(mouse) = mouse.get(&self.id) {
            if mouse.is_pressed() {
                if let Some(position) = self.get_relative(mouse.position) {
                    let center = match self.origin {
                        JoystickOrigin::Screen => point2(0.0, 0.0),
                        JoystickOrigin::Relative => self.get_relative(mouse.press_position).unwrap_or(point2(0.0, 0.0)),
                    };

                    let strength = (position.distance(center) * config::JOYSTICK_SENSITIVITY).min(1.0);
                    let angle = (position.y - center.y).atan2(position.x - center.x);
                    let direction = vec2(strength * angle.cos(), strength * angle.sin());
                    return Some((center, direction, strength));
                }
            }
        }

        None
    }

    fn get_relative(&self, position: Option<Point2<f32>>) -> Option<Point2<f32>> {
        position.map(|position| {
            Point2::new(
                position.x / self.area.x * 2.0 - 1.0,
                (position.y / self.area.y * 2.0 - 1.0) * (self.area.y / self.area.x),
            )
        })
    }
}
