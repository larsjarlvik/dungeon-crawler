use super::primitive;
use cgmath::*;

#[derive(Debug)]
pub struct Placeholder {
    pub name: String,
    pub position: Vector3<f32>,
}

impl Placeholder {
    pub fn new(gltf_mesh: &gltf::Mesh, primitive: &primitive::Primitive) -> Self {
        Self {
            name: gltf_mesh.name().expect("Placeholder is missing name!").to_string(),
            position: primitive.get_center(),
        }
    }
}
