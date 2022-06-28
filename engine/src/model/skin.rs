use cgmath::*;
use crate::config;

#[derive(Clone)]
pub struct Joint {
    pub inverse_bind_matrix: Matrix4<f32>,
    pub node_id: usize,
}

#[derive(Clone)]
pub struct Skin {
    pub joints: Vec<Joint>,
}

impl Skin {
    pub fn new(skin: &gltf::Skin, buffers: &Vec<gltf::buffer::Data>) -> Self {
        let joint_count = skin.joints().count();
        if joint_count > config::MAX_JOINT_COUNT {
            panic!("{} joints is more than {} allowed!", joint_count, config::MAX_JOINT_COUNT);
        }

        let inverse_bind_matrices = map_inverse_bind_matrices(skin, buffers);
        let node_ids = map_node_ids(skin);

        let joints = inverse_bind_matrices
            .iter()
            .zip(node_ids)
            .map(|(matrix, node_id)| Joint {
                inverse_bind_matrix: *matrix,
                node_id,
            })
            .collect::<Vec<_>>();

        Self { joints }
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
