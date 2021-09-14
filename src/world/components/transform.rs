use cgmath::*;
use specs::{Component, VecStorage};

pub struct Transform {
    pub translation: Vector3<f32>,
    pub rotation: f32,
    pub scale: Vector3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn from_translation(translation: Vector3<f32>) -> Self {
        Self {
            translation,
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}
