use cgmath::*;

#[derive(Clone)]
pub struct Node {
    pub local_transform: gltf::scene::Transform,
    pub global_transform_matrix: Matrix4<f32>,
    pub skin_index: Option<usize>,
    pub children_indices: Vec<usize>,
}

impl Node {
    pub fn new(node: &gltf::Node) -> Self {
        let local_transform = node.transform();
        let global_transform_matrix = compute_transform_matrix(&local_transform);
        let skin_index = node.skin().map(|s| s.index());
        let children_indices = node.children().map(|c| c.index()).collect::<Vec<_>>();

        Self {
            local_transform,
            global_transform_matrix,
            skin_index,
            children_indices,
        }
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        if let gltf::scene::Transform::Decomposed { rotation, scale, .. } = self.local_transform {
            self.local_transform = gltf::scene::Transform::Decomposed {
                translation: [translation.x, translation.y, translation.z],
                rotation,
                scale,
            }
        }
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        if let gltf::scene::Transform::Decomposed { translation, scale, .. } = self.local_transform {
            self.local_transform = gltf::scene::Transform::Decomposed {
                translation,
                rotation: [rotation.v.x, rotation.v.y, rotation.v.z, rotation.s],
                scale,
            }
        }
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        if let gltf::scene::Transform::Decomposed { translation, rotation, .. } = self.local_transform {
            self.local_transform = gltf::scene::Transform::Decomposed {
                translation,
                rotation,
                scale: [scale.x, scale.y, scale.z],
            }
        }
    }

    pub fn apply_transform(&mut self, transform: Matrix4<f32>) {
        let new_tranform = transform * compute_transform_matrix(&self.local_transform);
        self.global_transform_matrix = new_tranform;
    }
}

fn compute_transform_matrix(transform: &gltf::scene::Transform) -> Matrix4<f32> {
    match transform {
        gltf::scene::Transform::Matrix { matrix } => Matrix4::from(*matrix),
        gltf::scene::Transform::Decomposed {
            translation,
            rotation: [xr, yr, zr, wr],
            scale: [xs, ys, zs],
        } => {
            let translation = Matrix4::from_translation(Vector3::from(*translation));
            let rotation = Matrix4::from(Quaternion::new(*wr, *xr, *yr, *zr));
            let scale = Matrix4::from_nonuniform_scale(*xs, *ys, *zs);
            translation * rotation * scale
        }
    }
}
