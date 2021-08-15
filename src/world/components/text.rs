use cgmath::*;
use specs::{Component, VecStorage};

pub struct Text {
    pub text: String,
    pub position: Vector2<f32>,
}

impl Text {
    pub fn new(text: &str, position: Vector2<f32>) -> Self {
        Self {
            text: text.to_string(),
            position,
        }
    }
}

impl Component for Text {
    type Storage = VecStorage<Self>;
}
