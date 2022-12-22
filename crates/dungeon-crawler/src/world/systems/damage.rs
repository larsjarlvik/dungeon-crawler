use crate::world::components;
use bevy_ecs::prelude::*;
use cgmath::*;
use engine::collision::{Intersection, Polygon, PolygonMethods};
use rand::Rng;

pub fn damage(
    mut commands: Commands,
    attack_query: Query<(Entity, &components::Attack, &engine::ecs::components::Transform)>,
    mut target_query: Query<(
        &mut components::Stats,
        &components::Collision,
        &engine::ecs::components::Transform,
    )>,
) {
    let mut rng = rand::thread_rng();

    for (entity, attack, attack_transform) in attack_query.iter() {
        let attack_polygon = vec![vec2(
            attack_transform.translation.current.x,
            attack_transform.translation.current.z,
        )];

        for (mut target_stats, target, target_transform) in target_query.iter_mut() {
            // Avoid friendly fire
            if target_stats.team == attack.team {
                continue;
            }

            if did_hit(&attack_polygon, target, target_transform) {
                target_stats.health.changes.push(components::HealthChange::new(
                    -rng.gen_range(attack.damage.clone()).round(),
                    components::HealthChangeType::Once,
                ));

                break;
            }
        }

        commands.entity(entity).despawn();
    }
}

fn did_hit(attack: &Polygon, target: &components::Collision, target_transform: &engine::ecs::components::Transform) -> bool {
    let c = target
        .polygons
        .iter()
        .flat_map(move |p| p.transform(target_transform.translation.current, target_transform.rotation.current))
        .collect();

    !matches!(
        engine::collision::check_collision(attack, &c, Vector2::zero()),
        Intersection::None
    )
}
