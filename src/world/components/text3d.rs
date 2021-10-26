use cgmath::*;
use specs::{Component, VecStorage};

pub struct Text3d {
    pub text: String,
    pub position: Vector3<f32>,
    pub scale: f32,
}

impl Text3d {
    pub fn new(text: &str, position: Vector3<f32>, scale: f32) -> Self {
        Self {
            text: text.to_string(),
            position,
            scale,
        }
    }
}

impl Component for Text3d {
    type Storage = VecStorage<Self>;
}
