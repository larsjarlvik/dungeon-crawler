use cgmath::*;
use specs::{Component, VecStorage};

pub struct Light {
    pub direction: Option<Vector3<f32>>,
    pub color: Vector3<f32>,
    pub attenuation: Option<f32>,
}

impl Component for Light {
    type Storage = VecStorage<Self>;
}
