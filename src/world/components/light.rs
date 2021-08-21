use cgmath::*;
use specs::{Component, VecStorage};

pub struct Light {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub color: Vector3<f32>,
    pub attenuation: f32,
}

impl Component for Light {
    type Storage = VecStorage<Self>;
}
