use crate::world::*;
use cgmath::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = (
        Read<'a, resources::Time>,
        WriteStorage<'a, components::Animation>,
        WriteStorage<'a, components::Model>,
    );

    fn run(&mut self, (time, mut animation, mut model): Self::SystemData) {
        for (animation, model) in (&mut animation, &mut model).join() {
            animation.time += time.elapsed;

            // Animate
            let nodes = &mut model.nodes;
            let animations = &mut model.animations.iter_mut();
            for model_animation in animations {
                model_animation.animate(nodes, animation.time.as_millis() as f32 / 1000.0 % model_animation.total_time);
            }

            // Transform
            for (index, parent_index) in &model.depth_first_taversal_indices {
                let parent_transform = parent_index
                    .map(|id| {
                        let parent = &model.nodes[id];
                        parent.global_transform_matrix
                    })
                    .or(Matrix4::identity().into());

                if let Some(matrix) = parent_transform {
                    let node = &mut model.nodes[*index];
                    node.apply_transform(matrix);
                }
            }

            // Compute
            let transforms: Vec<(usize, Matrix4<f32>)> = model
                .nodes
                .iter()
                .filter(|n| n.skin_index.is_some())
                .map(|n| (n.skin_index.unwrap(), n.global_transform_matrix))
                .collect();

            for (index, transform) in transforms {
                model.skins[index].compute_joints_matrices(transform, &model.nodes);
            }
        }
    }
}
