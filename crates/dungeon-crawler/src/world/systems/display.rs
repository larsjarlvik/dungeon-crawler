use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;

pub fn display(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<(&engine::ecs::components::Transform, &components::Movement), With<components::UserControl>>,
        QueryState<(Entity, &components::Agressor, &engine::ecs::components::Transform)>,
        QueryState<(Entity, &components::Display)>,
    )>,
) {
    for (entity, _) in query.q2().iter() {
        commands.entity(entity).remove::<components::Display>();
    }

    let targets: Vec<(Entity, Vector3<f32>)> = query
        .q1()
        .iter()
        .map(|(entity, _, t)| (entity, t.translation.current.clone()))
        .collect();

    for (transform, movement) in query.q0().iter() {
        let target: Option<&(Entity, Vector3<f32>)> = targets
            .iter()
            .filter(|(_, target)| {
                if target.distance(transform.translation.current) > 5.0 {
                    return false;
                }

                let direction = target - transform.translation.current;
                let direction = direction.x.atan2(direction.z);
                (direction - movement.direction).abs() < 1.57
            })
            .next();

        if let Some(target) = target {
            commands.entity(target.0).insert(components::Display);
        }
    }
}
