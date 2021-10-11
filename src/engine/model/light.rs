use cgmath::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Light {
    pub name: String,
    pub radius: f32,
    pub intensity: f32,
    pub flicker: Option<f32>,
    pub color: Vector3<f32>,
    pub translation: Vector3<f32>,
}

impl Light {
    pub fn new(light: &gltf::khr_lights_punctual::Light, nodes: &Vec<gltf::Node>) -> Self {
        let mut translation = Vector3::zero();
        for n in nodes.iter() {
            let (t, _, _) = n.transform().decomposed();
            translation += Vector3::from(t);
        }

        let mut extras: HashMap<String, f32> = HashMap::new();
        if let Some(json) = light.extras() {
            extras = serde_json::from_str(json.get()).unwrap();
        }

        let flicker = if let Some(flicker) = extras.get("flicker") {
            Some(*flicker)
        } else {
            None
        };

        Self {
            name: light.name().unwrap().to_string(),
            radius: light.range().unwrap_or(5.0),
            intensity: light.intensity() / 20.0,
            flicker,
            color: Vector3::from(light.color()),
            translation: Vector3::from(translation),
        }
    }
}
