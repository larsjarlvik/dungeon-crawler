use super::node;
use cgmath::*;

#[derive(Clone)]
pub struct Joint {
    pub matrix: Matrix4<f32>,
    inverse_bind_matrix: Matrix4<f32>,
    node_id: usize,
}

#[derive(Clone)]
pub struct Skin {
    pub joints: Vec<Joint>,
}

impl Skin {
    pub fn new(skin: &gltf::Skin, buffers: &Vec<gltf::buffer::Data>) -> Self {
        let joint_count = skin.joints().count();
        if joint_count > 20 {
            panic!("{} joints is more than 20 allowed!", joint_count);
        }

        let inverse_bind_matrices = map_inverse_bind_matrices(skin, buffers);
        let node_ids = map_node_ids(skin);

        let joints = inverse_bind_matrices
            .iter()
            .zip(node_ids)
            .map(|(matrix, node_id)| Joint {
                inverse_bind_matrix: *matrix,
                matrix: Matrix4::identity(),
                node_id,
            })
            .collect::<Vec<_>>();

        Self { joints }
    }

    pub fn compute_joints_matrices(&mut self, transform: Matrix4<f32>, nodes: &[node::Node]) {
        self.joints.iter_mut().for_each(|joint| {
            let global_transform_inverse = transform.invert().expect("Transform matrix should be invertible");
            let node_transform = nodes[joint.node_id].global_transform_matrix;
            joint.matrix = global_transform_inverse * node_transform * joint.inverse_bind_matrix;
        });
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