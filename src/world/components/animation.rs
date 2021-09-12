use core::time;

use specs::{Component, VecStorage};

pub struct Animation {
    pub name: String,
    pub time: time::Duration,
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

impl Animation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            time: time::Duration::new(0, 0),
        }
    }
}
