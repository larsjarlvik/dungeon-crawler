use cgmath::*;
use std::collections::HashMap;

pub struct Transitions {
    asset_transitions: HashMap<String, Vector4<f32>>,
}

impl Transitions {
    pub fn new() -> Self {
        Self {
            asset_transitions: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &Option<String>, to: Vector4<f32>) -> Vector4<f32> {
        if let Some(key) = &key {
            let prev_val = *self.asset_transitions.get(key).unwrap_or(&to);
            let new_val = prev_val.lerp(to, 0.03);
            *self.asset_transitions.entry(key.clone()).or_insert(new_val) = new_val;
            return new_val;
        }

        to
    }
}
