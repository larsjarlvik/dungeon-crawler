use cgmath::*;
use specs::{Component, VecStorage};

pub struct Position(pub Vector3<f32>);

impl Component for Position {
    type Storage = VecStorage<Self>;
}
