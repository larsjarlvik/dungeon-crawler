use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Child {
    pub parent_id: u32,
}

impl Child {
    pub fn new(parent_id: u32) -> Self {
        Self { parent_id }
    }
}

impl Component for Child {
    type Storage = VecStorage<Self>;
}
