use cgmath::*;

#[derive(Clone)]
pub struct Node {
    pub local_transform: super::Transform,
    pub global_transform_matrix: Matrix4<f32>,
    pub skin_index: Option<usize>,
    pub children_indices: Vec<usize>,
}

impl Node {
    pub fn new(node: &gltf::Node) -> Self {
        let (translation, rotation, scale) = node.transform().decomposed();
        let local_transform = super::Transform {
            translation: Vector3::from(translation),
            rotation: Quaternion::from(rotation),
            scale: Vector3::from(scale),
        };

        let global_transform_matrix = local_transform.to_matrix();
        let skin_index = node.skin().map(|s| s.index());
        let children_indices = node.children().map(|c| c.index()).collect::<Vec<_>>();

        Self {
            local_transform,
            global_transform_matrix,
            skin_index,
            children_indices,
        }
    }

    pub fn apply_transform(&mut self, transform: Option<Matrix4<f32>>) {
        let new_tranform = if let Some(transform) = transform {
            transform * self.local_transform.to_matrix()
        } else {
            self.local_transform.to_matrix()
        };
        self.global_transform_matrix = new_tranform;
    }
}
