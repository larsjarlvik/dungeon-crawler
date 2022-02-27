use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Health {
    pub amount: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}
