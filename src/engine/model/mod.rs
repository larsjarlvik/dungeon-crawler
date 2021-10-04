use cgmath::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    usize,
};
pub mod animation;
mod interpolation;
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
    pub materials: Vec<material::Material>,
    pub collisions: HashMap<String, collision::Polygon>,
    pub animations: HashMap<String, animation::Animation>,
    pub depth_first_taversal_indices: Vec<(usize, Option<usize>)>,
}

impl GltfModel {
    pub fn new(ctx: &super::Context, bytes: &[u8]) -> Self {
        let (gltf, buffers, images) = gltf::import_slice(bytes).expect("Failed to import GLTF!");

        let mut meshes = vec![];
        let mut skins = vec![];
        let mut nodes = vec![];
        let mut collisions: HashMap<String, collision::Polygon> = HashMap::new();

        for mesh in gltf.meshes() {
            if let Some(mesh_name) = mesh.name() {
                dbg!(mesh_name);
                if mesh_name.contains("_col") {
                    let key = mesh_name.split("_").collect::<Vec<&str>>()[0].to_string();
                    let primitives: Vec<gltf::Primitive> = mesh.primitives().collect();
                    let mut polygon = build_collision_polygon(&primitives[0], &buffers);

                    match collisions.entry(key) {
                        Entry::Vacant(e) => {
                            e.insert(polygon);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().append(&mut polygon);
                        }
                    }
                } else {
                    meshes.insert(mesh.index(), mesh::Mesh::new(&mesh, &buffers));
                }
            }
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

fn build_collision_polygon(primitive: &gltf::Primitive, buffers: &Vec<gltf::buffer::Data>) -> collision::Polygon {
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let mut positions: Vec<Vector2<f32>> = reader
        .read_positions()
        .expect("No positions found!")
        .map(|p| vec2(p[0], p[2]))
        .collect();

    positions.sort_by(|a, b| {
        if a.x == b.x {
            a.y.partial_cmp(&b.y).unwrap()
        } else {
            a.x.partial_cmp(&b.x).unwrap()
        }
    });

    let mut upper_hull = vec![];
    for p in positions.iter() {
        while upper_hull.len() >= 2 {
            let q: Vector2<f32> = upper_hull[upper_hull.len() - 1];
            let r: Vector2<f32> = upper_hull[upper_hull.len() - 2];

            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                upper_hull.pop();
            } else {
                break;
            }
        }
        upper_hull.push(*p);
    }
    upper_hull.pop();

    let mut lower_hull = vec![];
    for p in positions.iter().rev() {
        while lower_hull.len() >= 2 {
            let q: Vector2<f32> = lower_hull[lower_hull.len() - 1];
            let r: Vector2<f32> = lower_hull[lower_hull.len() - 2];

            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                lower_hull.pop();
            } else {
                break;
            }
        }
        lower_hull.push(*p);
    }
    lower_hull.pop();

    if upper_hull.len() == 1 && lower_hull.len() == 1 && upper_hull[0].x == lower_hull[0].x && upper_hull[0].y == lower_hull[0].y {
        upper_hull
    } else {
        upper_hull.append(&mut lower_hull);
        upper_hull
    }
}
