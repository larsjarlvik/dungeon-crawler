use std::usize;

pub mod animation;
mod material;
mod mesh;
pub mod node;
mod primitive;
pub mod skin;
mod vertex;

pub use mesh::Mesh;
pub use primitive::Primitive;
pub use vertex::Vertex;

pub struct GltfModel {
    pub meshes: Vec<mesh::Mesh>,
    pub skins: Vec<skin::Skin>,
    pub nodes: Vec<node::Node>,
    pub materials: Vec<material::Material>,
    pub animations: Vec<animation::Animation>,
    pub depth_first_taversal_indices: Vec<(usize, Option<usize>)>,
}

impl GltfModel {
    pub fn new(ctx: &super::Context, bytes: &[u8]) -> Self {
        let (gltf, buffers, images) = gltf::import_slice(bytes).expect("Failed to import GLTF!");

        let mut meshes = vec![];
        let mut skins = vec![];
        let mut nodes = vec![];

        for mesh in gltf.meshes() {
            meshes.insert(mesh.index(), mesh::Mesh::new(&mesh, &buffers));
        }

        for skin in gltf.skins() {
            skins.insert(skin.index(), skin::Skin::new(&skin, &buffers));
        }

        for node in gltf.nodes() {
            nodes.insert(node.index(), node::Node::new(&node));
        }

        let roots_indices = gltf.default_scene().unwrap().nodes().map(|n| n.index()).collect::<Vec<_>>();
        let depth_first_taversal_indices = build_graph_run_indices(&roots_indices, &nodes);

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
            skins,
            nodes,
            materials,
            animations,
            depth_first_taversal_indices,
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

fn build_graph_run_indices(roots_indices: &[usize], nodes: &Vec<node::Node>) -> Vec<(usize, Option<usize>)> {
    let mut indices = Vec::new();
    for root_index in roots_indices {
        build_graph_run_indices_rec(nodes, *root_index, None, &mut indices);
    }
    indices
}

fn build_graph_run_indices_rec(
    nodes: &[node::Node],
    node_index: usize,
    parent_index: Option<usize>,
    indices: &mut Vec<(usize, Option<usize>)>,
) {
    indices.push((node_index, parent_index));
    for child_index in &nodes[node_index].children_indices {
        build_graph_run_indices_rec(nodes, *child_index, Some(node_index), indices);
    }
}
