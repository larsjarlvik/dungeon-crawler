use std::collections::HashMap;

use crate::engine::{self, model};
use specs::{Component, VecStorage};

pub struct Model {
    pub depth_first_taversal_indices: Vec<(usize, Option<usize>)>,
    pub model: engine::pipelines::model::Model,
    pub nodes: Vec<model::node::Node>,
    pub skins: Vec<model::skin::Skin>,
    pub animations: HashMap<String, model::animation::Animation>,
}

impl Model {
    pub fn new(engine: &engine::Engine, gltf: &model::GltfModel, name: &str) -> Self {
        let model = engine.get_mesh(gltf, name);
        Self {
            model,
            depth_first_taversal_indices: gltf.depth_first_taversal_indices.clone(),
            nodes: gltf.nodes.clone(),
            skins: gltf.skins.clone(),
            animations: gltf.animations.clone(),
        }
    }
}

impl Component for Model {
    type Storage = VecStorage<Self>;
}
