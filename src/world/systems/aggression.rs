use crate::world::*;
use bevy_ecs::prelude::*;

pub fn aggression(
    mut query: QuerySet<(
        QueryState<(&components::Target, &components::Transform)>,
        QueryState<(
            &mut components::Agressor,
            &mut components::Movement,
            &mut components::Action,
            &components::Transform,
        )>,
    )>,
) {
    let targets: Vec<Vector3<f32>> = query.q0().iter().map(|t| t.1.translation.current.clone()).collect();

    for (mut agressor, mut movement, mut action, transform) in query.q1().iter_mut() {
        for target_transform in targets.iter() {
            let distance = transform.translation.current.distance(*target_transform);
            let range = if agressor.is_aggressive {
                agressor.end_range
            } else {
                agressor.start_range
            };

            if action.current == components::CurrentAction::None {
                movement.towards(target_transform - transform.translation.current);

                if distance < 1.5 {
                    action.set_action(components::CurrentAction::Attack(2.0), 1.0, 0.35, false);
                } else if transform.translation.current.distance(*target_transform) < range {
                    movement.velocity = 0.07;
                    agressor.is_aggressive = true;
                }
            }
        }
    }
}
