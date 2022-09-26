use super::primitive;
use crate::bounding_box;

#[derive(Clone)]
pub struct Mesh {
    pub name: String,
    pub primitives: Vec<primitive::Primitive>,
    pub bounding_box: bounding_box::BoundingBox,
}

impl Mesh {
    pub fn new(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Self {
        let mut primitives = vec![];
        let name = String::from(mesh.name().unwrap());
        let mut bounding_box = bounding_box::BoundingBox::default();

        for gltf_primitive in mesh.primitives() {
            let primitive = primitive::Primitive::new(buffers, &gltf_primitive);
            bounding_box = bounding_box.grow(&primitive.bounding_box);
            primitives.push(primitive);
        }

        Mesh {
            name,
            bounding_box,
            primitives,
        }
    }
}
