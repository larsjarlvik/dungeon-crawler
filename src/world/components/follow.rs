use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Follow;

impl Component for Follow {
    type Storage = NullStorage<Self>;
}
