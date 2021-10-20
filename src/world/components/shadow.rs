use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Shadow;

impl Component for Shadow {
    type Storage = NullStorage<Self>;
}
