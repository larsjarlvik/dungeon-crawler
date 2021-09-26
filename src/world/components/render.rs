use specs::{Component, VecStorage};

pub struct Render {
    pub cull_frustum: bool,
}

impl Component for Render {
    type Storage = VecStorage<Self>;
}
