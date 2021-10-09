use cgmath::*;

#[derive(Debug)]
pub struct Light {
    pub name: String,
    pub radius: f32,
    pub translation: Vector3<f32>,
}

impl Light {
    pub fn new(node: &gltf::Node) -> Self {
        let (translation, _, scale) = node.transform().decomposed();

        Self {
            name: node.name().unwrap().to_string(),
            radius: scale[0].max(scale[1]).max(scale[2]),
            translation: Vector3::from(translation),
        }
    }
}
