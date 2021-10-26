use crate::utils::{InterpolatedValue};
use cgmath::*;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Transform2d {
    pub translation: InterpolatedValue<Vector2<f32>>,
    pub scale: InterpolatedValue<Vector2<f32>>,
}

impl Component for Transform2d {
    type Storage = VecStorage<Self>;
}

impl Transform2d {
    pub fn from_translation_scale(translation: Vector2<f32>, scale: f32) -> Self {
        Self {
            translation: InterpolatedValue::new(translation),
            scale: InterpolatedValue::new(vec2(scale, scale)),
        }
    }
}
