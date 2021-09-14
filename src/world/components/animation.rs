use cgmath::{Matrix4, SquareMatrix};
use core::time;
use specs::{Component, VecStorage};

pub struct Channel {
    pub name: String,
    pub time: time::Duration,
}

pub struct Animation {
    pub channels: Vec<Channel>,
    pub joint_matrices: Vec<Matrix4<f32>>,
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

impl Animation {
    pub fn new(names: Vec<&str>) -> Self {
        Self {
            joint_matrices: vec![Matrix4::identity(); 20],
            channels: names
                .iter()
                .map(|n| Channel {
                    name: n.to_string(),
                    time: time::Duration::new(0, 0),
                })
                .collect(),
        }
    }
}
