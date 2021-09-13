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
        for (animations, model) in (&mut animation, &mut model).join() {
            // Animate
            let mut dirty = false;

            for channel in animations.channels.iter_mut() {
                channel.time += time.elapsed;

                let active_animation = model
                    .animations
                    .get_mut(&channel.name)
                    .expect(format!("Could not find animation: {}", &channel.name).as_str());

                if active_animation.animate(
                    &mut model.nodes,
                    channel.time.as_millis() as f32 / 1000.0 % active_animation.total_time,
                ) {
                    dirty = true;
                }
            }

            if dirty {
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
}
