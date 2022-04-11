use crate::{
    engine::{
        self,
        collision::{Intersection, Polygon, PolygonMethods},
    },
    world::components,
};
use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::DespawnRecursiveExt;
use cgmath::*;
use rand::Rng;

pub fn damage(
    mut commands: Commands,
    attack_query: Query<(Entity, &components::Attack, &components::Transform)>,
    mut target_query: Query<(&mut components::Health, &components::Collision, &components::Transform)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, attack, transform) in attack_query.iter() {
        for (mut health, collision, target_transform) in target_query.iter_mut() {
            if collision.key == attack.collision_key {
                continue;
            }

            let collider: Vec<Polygon> = collision
                .polygons
                .iter()
                .map(|p| p.transform(target_transform.translation.current, target_transform.rotation.current))
                .collect();

            for polygon in collider {
                if did_hit(&polygon, collision, transform) {
                    health.changes.push(components::HealthChange::new(
                        -rng.gen_range(attack.min..attack.max).round(),
                        components::HealthChangeType::Once,
                    ));
                    break;
                }
            }
        }

        commands.entity(entity).despawn_recursive();
    }
}

fn did_hit(collider: &Polygon, collision: &components::Collision, transform: &components::Transform) -> bool {
    let c = collision
        .polygons
        .iter()
        .flat_map(move |p| p.transform(transform.translation.current, transform.rotation.current))
        .collect();

    let result = engine::collision::check_collision(&collider, &c, Vector2::zero());
    match result {
        Intersection::None => {}
        _ => {
            return true;
        }
    };

    return false;
}
