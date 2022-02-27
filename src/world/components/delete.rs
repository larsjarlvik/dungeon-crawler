use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Delete;

impl Component for Delete {
    type Storage = NullStorage<Self>;
}
