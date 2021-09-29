use crate::engine::collision::Polygon;
use specs::{Component, VecStorage};

pub struct Collision {
    pub polygon: Polygon,
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}

impl Collision {
    pub fn new(polygon: Polygon) -> Self {
        Self { polygon }
    }
}
