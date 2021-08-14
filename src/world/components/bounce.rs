use cgmath::Vector3;
use specs::{Component, VecStorage};

pub struct Bouce(pub Vector3<f32>);

impl Component for Bouce {
    type Storage = VecStorage<Self>;
}
