use specs::{Component, NullStorage};

#[derive(Default)]
pub struct UserControl;

impl Component for UserControl {
    type Storage = NullStorage<Self>;
}
