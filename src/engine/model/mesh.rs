use std::convert::TryInto;

use super::primitive;
use cgmath::*;

pub struct Joint {
    matrix: Matrix4<f32>,
    inverse_bind_matrix: Matrix4<f32>,
    node_id: usize,
}

pub struct Mesh {
    pub index: usize,
    pub children: Vec<usize>,
    pub name: String,
    pub primitives: Vec<primitive::Primitive>,
    pub skin: Vec<Joint>,
}

impl Mesh {
    pub fn new<'a>(node: gltf::Node, buffers: &Vec<gltf::buffer::Data>) -> Self {
        let mut primitives = vec![];
        let mesh = node.mesh().unwrap();

        let name = String::from(mesh.name().unwrap());
        let mut skin = vec![];

        for gltf_primitive in mesh.primitives() {
            let primitive = primitive::Primitive::new(buffers, &gltf_primitive);
            primitives.push(primitive);
        }

        if let Some(gltf_skin) = &node.skin() {
            let joint_count = gltf_skin.joints().count();
            if joint_count > 20 {
                panic!("{} joints is more than 20 allowed!", joint_count);
            }

            let inverse_bind_matrices = map_inverse_bind_matrices(gltf_skin, buffers);
            let node_ids = map_node_ids(gltf_skin);

            skin = inverse_bind_matrices
                .iter()
                .zip(node_ids)
                .map(|(matrix, node_id)| Joint {
                    inverse_bind_matrix: *matrix,
                    matrix: Matrix4::identity(),
                    node_id,
                })
                .collect::<Vec<_>>();
        }

        Mesh {
            index: node.index(),
            children: node.children().into_iter().map(|child| child.index()).collect(),
            name,
            primitives,
            skin,
        }
    }

    pub fn get_transforms(&self) -> [[[f32; 4]; 4]; 20] {
        let mut existing: Vec<[[f32; 4]; 4]> = self.skin.iter().map(|joint| joint.inverse_bind_matrix.into()).collect();
        let zeros: Vec<[[f32; 4]; 4]> = vec![Matrix4::zero().into(); 20 - existing.len()];
        existing.extend(zeros);
        existing.try_into().expect("Incorrect length!")
    }
}

fn map_inverse_bind_matrices(gltf_skin: &gltf::Skin, data: &[gltf::buffer::Data]) -> Vec<Matrix4<f32>> {
    let iter = gltf_skin
        .reader(|buffer| Some(&data[buffer.index()]))
        .read_inverse_bind_matrices()
        .expect("IBM reader not found for skin");
    iter.map(Matrix4::from).collect::<Vec<_>>()
}

fn map_node_ids(gltf_skin: &gltf::Skin) -> Vec<usize> {
    gltf_skin.joints().map(|node| node.index()).collect::<Vec<_>>()
}
