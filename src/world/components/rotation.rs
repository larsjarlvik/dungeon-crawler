use cgmath::*;
use specs::{Component, VecStorage};

pub struct Rotation(pub Vector3<f32>);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
