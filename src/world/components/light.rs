use cgmath::*;
use specs::{Component, VecStorage};

pub struct Light {
    pub offset: Vector3<f32>,
    pub color: Vector3<f32>,
    pub intensity: f32,
    pub radius: Option<f32>,
}

impl Component for Light {
    type Storage = VecStorage<Self>;
}
