use crate::world::*;
use cgmath::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = (
        Read<'a, resources::Time>,
        WriteStorage<'a, components::Animation>,
        ReadStorage<'a, components::Model>,
    );

    fn run(&mut self, (time, mut animation, model): Self::SystemData) {
        for (animations, model) in (&mut animation, &model).join() {
            animations.joint_matrices = vec![Matrix4::identity(); 20];

            // Animate
            for channel in animations.channels.iter_mut() {
                channel.time += time.elapsed;
                animate(model, channel, &mut animations.joint_matrices);
            }
        }
    }
}

fn animate(model: &components::Model, channel: &mut components::animation::Channel, joint_matrices: &mut Vec<Matrix4<f32>>) {
    let animation = model
        .animations
        .get(&channel.name)
        .expect(format!("Could not find animation: {}", &channel.name).as_str());

    let mut nodes = model.nodes.clone();
    if animation.animate_nodes(&mut nodes, channel.time.as_millis() as f32 / 1000.0 % animation.total_time) {
        // Transform
        for (index, parent_index) in &model.depth_first_taversal_indices {
            let parent_transform = parent_index
                .map(|id| {
                    let parent = &nodes[id];
                    parent.global_transform_matrix
                })
                .or(Matrix4::identity().into());

            if let Some(matrix) = parent_transform {
                let node = &mut nodes[*index];
                node.apply_transform(matrix);
            }
        }

        // Compute
        let transforms: Vec<(usize, Matrix4<f32>)> = nodes
            .iter()
            .filter(|n| n.skin_index.is_some())
            .map(|n| (n.skin_index.unwrap(), n.global_transform_matrix))
            .collect();

        for (s_index, transform) in transforms {
            model.skins[s_index].joints.iter().enumerate().for_each(|(j_index, joint)| {
                let global_transform_inverse = transform.invert().expect("Transform matrix should be invertible");
                let node_transform = nodes[joint.node_id].global_transform_matrix;

                joint_matrices[j_index] = joint_matrices[j_index] * global_transform_inverse * node_transform * joint.inverse_bind_matrix;
            });
        }
    }
}
