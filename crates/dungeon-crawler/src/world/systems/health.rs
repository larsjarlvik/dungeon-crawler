use crate::{config, world::components};
use bevy_ecs::prelude::*;
use bevy_hierarchy::*;

pub fn health(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<(Entity, &mut components::Stats, Option<&mut components::ActionExecutor>)>,
        Query<&mut components::Stats, With<components::UserControl>>,
    )>,
) {
    let mut total_experience = vec![];

    for (entity, mut stats, mut action) in query.p0().iter_mut() {
        let previous = stats.health.get();
        if previous >= 0.0 {
            stats.health.changes = stats
                .health
                .changes
                .clone()
                .into_iter()
                .filter(|change| match change.change_type {
                    components::HealthChangeType::Once => {
                        stats.health.set(previous + change.amount);
                        false
                    }
                    components::HealthChangeType::Forever => {
                        if previous > 0.0 {
                            stats.health.set(previous + change.amount / config::UPDATES_PER_SECOND);
                            true
                        } else {
                            false
                        }
                    }
                    components::HealthChangeType::OverTime(length) => {
                        if previous > 0.0 && previous < stats.get_base_health() {
                            stats.health.set(previous + change.amount / config::UPDATES_PER_SECOND);
                            change.start.elapsed() < length
                        } else {
                            false
                        }
                    }
                })
                .collect();
        }

        let limited_health = stats.health.get().clamp(0.0, stats.get_base_health());
        stats.health.set(limited_health);

        if stats.health.get() < previous {
            if let Some(action) = &mut action {
                if stats.health.get() <= 0.0 {
                    action.set_action(components::Action::Death, 100.0, 0.0);
                    total_experience.push((stats.get_kill_experience(), stats.get_level()));

                    commands
                        .entity(entity)
                        .remove::<(components::Agressor, components::Target, components::Collision)>();
                } else {
                    action.set_action(components::Action::Hit, stats.get_recovery_time(), 0.0);
                }
            } else if stats.health.get() <= 0.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    for mut stats in query.p1().iter_mut() {
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
