use cgmath::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    usize,
};
pub mod animation;
mod interpolation;
mod light;
mod material;
mod mesh;
pub mod node;
mod primitive;
pub mod skin;
mod vertex;
use super::collision;
pub use mesh::Mesh;
pub use primitive::Primitive;
pub use vertex::Vertex;

pub struct GltfModel {
    pub meshes: Vec<mesh::Mesh>,
    pub skins: Vec<skin::Skin>,
    pub nodes: Vec<node::Node>,
    pub lights: Vec<light::Light>,
    pub materials: Vec<material::Material>,
    pub collisions: HashMap<String, Vec<collision::Polygon>>,
    pub animations: HashMap<String, animation::Animation>,
    pub depth_first_taversal_indices: Vec<(usize, Option<usize>)>,
}

impl GltfModel {
    pub fn new(ctx: &super::Context, bytes: &[u8]) -> Self {
        let (gltf, buffers, images) = gltf::import_slice(bytes).expect("Failed to import GLTF!");

        let mut meshes = vec![];
        let mut skins = vec![];
        let mut nodes = vec![];
        let mut collisions: HashMap<String, Vec<collision::Polygon>> = HashMap::new();
        let mut lights = vec![];

        for mesh in gltf.meshes() {
            if let Some(mesh_name) = mesh.name() {
                if mesh_name.split("_").any(|w| w == "col") {
                    let key = mesh_name.split("_").collect::<Vec<&str>>()[0].to_string();
                    let primitives: Vec<gltf::Primitive> = mesh.primitives().collect();
                    let mut polygons = build_collision_polygon(&primitives[0], &buffers);

                    match collisions.entry(key) {
                        Entry::Vacant(e) => {
                            e.insert(polygons);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().append(&mut polygons);
                        }
                    }
                }

                meshes.insert(mesh.index(), mesh::Mesh::new(&mesh, &buffers));
            }
        }

        for skin in gltf.skins() {
            skins.insert(skin.index(), skin::Skin::new(&skin, &buffers));
        }

        for node in gltf.nodes() {
            nodes.insert(node.index(), node::Node::new(&node));

            if let Some(light) = node.light() {
                let n = gltf.nodes().filter(|n| n.name() == light.name()).collect();
                lights.push(light::Light::new(&light, &n));
            }
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
            .map(|animation| {
                (
                    animation.name().unwrap().to_string(),
                    animation::Animation::new(animation, &buffers),
                )
            })
            .collect();

        Self {
            meshes,
            skins,
            nodes,
            materials,
            collisions,
            animations,
            lights,
            depth_first_taversal_indices,
        }
    }

    pub fn get_mesh_by_name(&self, name: &str) -> &mesh::Mesh {
        self.meshes
            .iter()
            .find(|m| m.name == name)
            .expect(format!("Failed to find mesh: {0}!", name).as_str())
    }

    pub fn get_material(&self, material: Option<usize>) -> Option<&material::Material> {
        if let Some(material) = material {
            return Some(&self.materials[material]);
        }

        None
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

fn build_collision_polygon(primitive: &gltf::Primitive, buffers: &Vec<gltf::buffer::Data>) -> Vec<collision::Polygon> {
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let positions: Vec<Vector2<f32>> = reader
        .read_positions()
        .expect("No positions found!")
        .map(|p| vec2(p[0], p[2]))
        .collect();

    let indices = reader.read_indices().expect("No indices found!").into_u32().collect::<Vec<u32>>();
    let mut polygons = vec![];
    for i in (0..indices.len()).step_by(3) {
        polygons.push(vec![
            positions[indices[i] as usize],
            positions[indices[i + 1] as usize],
            positions[indices[i + 2] as usize],
        ]);
    }

    polygons
}
