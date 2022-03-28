use crate::{
    engine::collision::{Intersection, Polygon, PolygonMethods},
    world::*,
};
use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::DespawnRecursiveExt;

pub fn action(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<(
            &mut components::Action,
            &components::Movement,
            &components::Collider,
            &components::Transform,
        )>,
        QueryState<(
            Entity,
            &mut components::Health,
            &components::Collision,
            &components::Transform,
            Option<&mut components::Action>,
        )>,
    )>,
) {
    let mut hits = vec![];

    for (mut action, movement, collider, transform) in query.q0().iter_mut() {
        match action.current {
            components::CurrentAction::Attack(dmg) => {
                if action.should_execute() {
                    let collider: Vec<Polygon> = collider
                        .polygons
                        .iter()
                        .map(|p| p.transform(transform.translation.current, transform.rotation.current))
                        .collect();
                    let velocity_dir = vec2(movement.direction.sin(), movement.direction.cos());
                    hits.push((collider, velocity_dir, dmg));
                }
            }
            _ => {}
        }

        if action.set.elapsed().as_secs_f32() > action.length {
            action.reset();
        }
    }

    for (entity, mut health, collision, transform, mut action) in query.q1().iter_mut() {
        for (collider, velocity_dir, dmg) in hits.iter() {
            for polygon in collider {
                if did_hit(&polygon, collision, transform, *velocity_dir) {
                    health.amount -= dmg;

                    if let Some(action) = &mut action {
                        if health.amount <= 0.0 {
                            action.set_action(components::CurrentAction::Death, 100.0, 0.5, true);
                        } else {
                            action.set_action(components::CurrentAction::Hit, 1.0, 0.5, true);
                        }
                    } else if health.amount <= 0.0 {
                        commands.entity(entity).despawn_recursive();
                    }

                    break;
                }
            }
        }
    }
}

fn did_hit(collider: &Polygon, collision: &components::Collision, transform: &components::Transform, velocity_dir: Vector2<f32>) -> bool {
    let c = collision
        .polygons
        .iter()
        .flat_map(move |p| p.transform(transform.translation.current, transform.rotation.current))
        .collect();

    let result = engine::collision::check_collision(&collider, &c, velocity_dir);
    match result {
        Intersection::None => {}
        _ => {
            return true;
        }
    };

    return false;
}
