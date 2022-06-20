use crate::{config, world::components};
use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::DespawnRecursiveExt;

pub fn health(mut commands: Commands, mut query: Query<(Entity, &mut components::Health, Option<&mut components::Action>)>) {
    for (entity, mut health, mut action) in query.iter_mut() {
        let previous = health.current;

        health.changes = health
            .changes
            .clone()
            .into_iter()
            .filter(|change| match change.change_type {
                components::HealthChangeType::Once => {
                    health.current += change.amount;
                    false
                }
                components::HealthChangeType::Forever => {
                    health.current += change.amount / config::UPDATES_PER_SECOND;
                    true
                }
                components::HealthChangeType::OverTime(length) => {
                    health.current += change.amount / config::UPDATES_PER_SECOND;
                    change.start.elapsed() < length
                }
            })
            .collect();

        health.current = health.current.min(health.max).max(0.0);

        if health.current < previous {
            if let Some(action) = &mut action {
                if health.current <= 0.0 {
                    action.set_action(components::CurrentAction::Death, 100.0, 0.0, true);
                    commands
                        .entity(entity)
                        .remove_bundle::<(components::Agressor, components::Target, components::Collision)>();
                } else {
                    action.set_action(components::CurrentAction::Hit, 0.5, 0.0, true);
                }
            } else if health.current <= 0.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
