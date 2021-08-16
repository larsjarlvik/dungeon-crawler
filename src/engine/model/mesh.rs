use super::primitive;

pub struct Mesh {
    pub name: String,
    pub primitives: Vec<primitive::Primitive>,
}

impl Mesh {
    pub fn new<'a>(mesh: gltf::Mesh<'_>, buffers: &Vec<gltf::buffer::Data>) -> Self {
        let mut primitives = vec![];
        let name = String::from(mesh.name().unwrap());

        for gltf_primitive in mesh.primitives() {
            let primitive = primitive::Primitive::new(buffers, &gltf_primitive);
            primitives.push(primitive);
        }

        Mesh { name, primitives }
    }
}
