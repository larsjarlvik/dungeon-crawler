use crate::{config, world::components};
use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::DespawnRecursiveExt;

pub fn health(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<(Entity, &mut components::Stats, Option<&mut components::Action>)>,
        QueryState<&mut components::Stats>,
    )>,
) {
    let mut total_experience = vec![];

    for (entity, mut stats, mut action) in query.q0().iter_mut() {
        let previous = stats.health.current;

        stats.health.changes = stats
            .health
            .changes
            .clone()
            .into_iter()
            .filter(|change| match change.change_type {
                components::HealthChangeType::Once => {
                    stats.health.current += change.amount;
                    false
                }
                components::HealthChangeType::Forever => {
                    stats.health.current += change.amount / config::UPDATES_PER_SECOND;
                    true
                }
                components::HealthChangeType::OverTime(length) => {
                    stats.health.current += change.amount / config::UPDATES_PER_SECOND;
                    change.start.elapsed() < length
                }
            })
            .collect();

        stats.health.current = stats.health.current.min(stats.get_base_health()).max(0.0);

        if stats.health.current < previous {
            if let Some(action) = &mut action {
                if stats.health.current <= 0.0 {
                    action.set_action(components::CurrentAction::Death, 100.0, 0.0, true);
                    total_experience.push((stats.get_kill_experience(), stats.get_level()));

                    commands
                        .entity(entity)
                        .remove_bundle::<(components::Agressor, components::Target, components::Collision)>();
                } else {
                    action.set_action(components::CurrentAction::Hit, stats.get_recovery_time(), 0.25, true);
                }
            } else if stats.health.current <= 0.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    for mut stats in query.q1().iter_mut() {
        let level = stats.get_level();

        for (exp, kill_level) in total_experience.iter() {
            stats.experience += (*exp as f32 + (*exp as f32 * (*kill_level as f32 / level as f32)).powf(1.2)) as u32;
        }

        if stats.get_level() > level {
            stats.level_up();
        }

        total_experience.clear();
    }
}
