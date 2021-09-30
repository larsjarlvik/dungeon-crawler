use crate::engine::collision::Polygon;
use specs::{Component, VecStorage};

pub struct Collision {
    pub polygons: Vec<Polygon>,
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}

impl Collision {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }
}
