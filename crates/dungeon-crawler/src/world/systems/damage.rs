use crate::world::components;
use bevy_ecs::prelude::*;
use cgmath::*;
use engine::collision::PolygonMethods;
use rand::Rng;

pub fn damage(
    mut commands: Commands,
    attack_query: Query<(Entity, &components::Attack, &engine::ecs::components::Transform)>,
    mut target_query: Query<(
        Entity,
        &mut components::Stats,
        &components::Collision,
        &engine::ecs::components::Transform,
    )>,
) {
    if attack_query.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    let targets: Vec<(Entity, usize, Vec<Vector2<f32>>)> = target_query
        .iter()
        .map(|(e, stats, collision, transform)| {
            let polygons = collision
                .polygons
                .iter()
                .flat_map(|p| p.transform(transform.translation.current, transform.rotation.current))
                .collect();
            (e, stats.team, polygons)
        })
        .collect();

    for (entity, attack, attack_transform) in attack_query.iter() {
        let attack_center = vec2(attack_transform.translation.current.x, attack_transform.translation.current.z);

        for (target_entity, team, polygons) in &targets {
            // Avoid friendly fire
            if *team == attack.team {
                continue;
            }

            if engine::collision::check_collision_circle(polygons, attack_center, attack.radius) {
                if let Ok((_, mut target_stats, _, _)) = target_query.get_mut(*target_entity) {
                    target_stats.health.changes.push(components::HealthChange::new(
                        -rng.gen_range(attack.damage.clone()).round(),
                        components::HealthChangeType::Once,
                    ));
                }
            }
        }

        commands.entity(entity).despawn();
    }
}
