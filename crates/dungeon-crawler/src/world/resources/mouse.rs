use cgmath::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum PressState {
    Released(bool),
    Pressed(bool),
}

#[derive(Debug, Clone)]
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
        self.position = Some(position);
        if self.press_position.is_none() {
            self.press_position = Some(position);
            dbg!(self.press_position);
        }
    }

    pub fn press(&mut self, touch: bool, pressed: bool) {
        self.touch = touch;
        match pressed {
            true => self.state = PressState::Pressed(false),
            false => {
                self.state = PressState::Released(false);
                self.press_position = None;
            }
        };
    }
}
