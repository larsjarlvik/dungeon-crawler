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

pub fn damage(
    mut commands: Commands,
    attack_query: Query<(Entity, &components::Attack, &components::Transform)>,
    mut target_query: Query<(
        Entity,
        &mut components::Health,
        Option<&mut components::Action>,
        &components::Collision,
        &components::Transform,
    )>,
) {
    for (entity, attack, transform) in attack_query.iter() {
        for (target_entity, mut health, mut action, collision, target_transform) in target_query.iter_mut() {
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
                    health.current -= attack.damage;

                    if let Some(action) = &mut action {
                        if health.current <= 0.0 {
                            action.set_action(components::CurrentAction::Death, 100.0, 0.5, true);
                            commands
                                .entity(target_entity)
                                .remove_bundle::<(components::Agressor, components::Target, components::Collision)>();
                        } else {
                            // TODO: Damage
                            action.set_action(components::CurrentAction::Hit, 0.5, 0.5, true);
                        }
                    } else if health.current <= 0.0 {
                        commands.entity(target_entity).despawn_recursive();
                    }

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
