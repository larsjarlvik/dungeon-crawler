mod animation;
mod material;
mod mesh;
mod primitive;
mod vertex;

pub use mesh::Mesh;
pub use primitive::Primitive;
pub use vertex::Vertex;

pub struct GltfModel {
    pub meshes: Vec<mesh::Mesh>,
    pub materials: Vec<material::Material>,
    pub animations: Vec<animation::Animation>,
}

impl GltfModel {
    pub fn new(ctx: &super::Context, bytes: &[u8]) -> Self {
        let (gltf, buffers, images) = gltf::import_slice(bytes).expect("Failed to import GLTF!");

        let meshes = gltf
            .nodes()
            .filter(|n| n.mesh().is_some())
            .map(|node| mesh::Mesh::new(node, &buffers))
            .collect();

        let materials = gltf
            .materials()
            .into_iter()
            .map(|material| material::Material::new(ctx, &material, &images))
            .collect();

        let animations = gltf
            .animations()
            .into_iter()
            .map(|animation| animation::Animation::new(animation, &buffers))
            .collect();

        Self {
            meshes,
            materials,
            animations,
        }
    }

    pub fn get_mesh_by_name(&self, name: &str) -> &mesh::Mesh {
        self.meshes
            .iter()
            .find(|m| m.name == name)
            .expect(format!("Failed to find mesh: {0}!", name).as_str())
    }

    pub fn get_material(&self, material: usize) -> &material::Material {
        &self.materials[material]
    }
}
