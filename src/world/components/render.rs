use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Render;

impl Component for Render {
    type Storage = NullStorage<Self>;
}
