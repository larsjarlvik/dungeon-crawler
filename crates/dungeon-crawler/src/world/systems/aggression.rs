use crate::world::*;
use bevy_ecs::prelude::*;

pub fn aggression(
    mut query: ParamSet<(
        Query<(&components::Target, &engine::ecs::components::Transform)>,
        Query<(
            &mut components::Agressor,
            &mut components::Movement,
            &mut components::Action,
            &components::Stats,
            Option<&components::Weapon>,
            &engine::ecs::components::Transform,
        )>,
    )>,
) {
    let targets: Vec<Vector3<f32>> = query.p0().iter().map(|t| t.1.translation.current.clone()).collect();

    for (mut agressor, mut movement, mut action, stats, weapon, transform) in query.p1().iter_mut() {
        for target_transform in targets.iter() {
            let distance = transform.translation.current.distance(*target_transform);
            let range = if agressor.is_aggressive {
                agressor.end_range
            } else {
                agressor.start_range
            };

            if action.current == components::CurrentAction::None {
                movement.towards(target_transform - transform.translation.current);

                // TODO: Attack range
                if distance < 1.0 {
                    if let Some(weapon) = weapon {
                        action.set_action(
                            components::CurrentAction::Attack,
                            weapon.time * stats.get_attack_time(),
                            0.3,
                            false,
                        );
                    }
                } else if transform.translation.current.distance(*target_transform) < range {
                    movement.velocity = 0.07;
                    agressor.is_aggressive = true;
                }
            }
        }
    }
}
