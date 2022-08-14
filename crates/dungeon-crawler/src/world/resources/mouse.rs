use cgmath::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum PressState {
    Released(bool),
    Pressed(bool),
}

#[derive(Debug)]
pub struct MouseButton {
    pub press_position: Option<Point2<f32>>,
    pub position: Option<Point2<f32>>,
    pub state: PressState,
    pub touch: bool,
}

impl MouseButton {
    pub fn new() -> Self {
        Self {
            press_position: None,
            position: None,
            state: PressState::Released(true),
            touch: false,
        }
    }

    pub fn is_pressed(&self) -> bool {
        match self.state {
            PressState::Released(_) => false,
            PressState::Pressed(_) => true,
        }
    }

    pub fn mouse_move(&mut self, position: Point2<f32>) {
        if self.press_position.is_none() {
            self.press_position = Some(position);
        }

        self.position = Some(position);
    }

    pub fn press(&mut self, touch: bool, pressed: bool) {
        self.touch = touch;
        match pressed {
            true => self.state = PressState::Pressed(false),
            false => self.state = PressState::Released(false),
        };
    }

    pub fn get_relative(&self, width: u32, height: u32) -> Option<Point2<f32>> {
        if let Some(position) = self.position {
            Some(Point2::new(
                position.x / width as f32 * 2.0 - 1.0,
                position.y / height as f32 * 2.0 - 1.0,
            ))
        } else {
            None
        }
    }
}
