use crate::{
    engine::collision::{Intersection, Polygon, PolygonMethods},
    world::*,
};
use bevy_ecs::prelude::*;

pub fn action(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<(
            &mut components::Action,
            &components::Movement,
            &components::Collider,
            &components::Transform,
        )>,
        QueryState<(Entity, &mut components::Health, &components::Collision, &components::Transform)>,
    )>,
) {
    let mut hits = vec![];

    for (mut action, movement, collider, transform) in query.q0().iter_mut() {
        match action.current {
            components::CurrentAction::Attack(dmg) => {
                let collider: Vec<Polygon> = collider
                    .polygons
                    .iter()
                    .map(|p| p.transform(transform.translation.current, transform.rotation.current))
                    .collect();
                let velocity_dir = vec2(movement.direction.sin(), movement.direction.cos());

                hits.push((collider, velocity_dir, dmg));
            }
            _ => {}
        }

        if let Some(action_changed) = action.set {
            if action_changed.elapsed().as_secs_f32() > action.length {
                action.reset();
            }
        }
    }

    for (entity, mut health, collision, transform) in query.q1().iter_mut() {
        if health.amount > 0.0 {
            for (collider, velocity_dir, dmg) in hits.iter() {
                for polygon in collider {
                    attack(&polygon, &mut health, collision, transform, *velocity_dir, *dmg);
                    if health.amount <= 0.0 {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

fn attack(
    collider: &Polygon,
    health: &mut components::Health,
    collision: &components::Collision,
    transform: &components::Transform,
    velocity_dir: Vector2<f32>,
    dmg: f32,
) -> bool {
    let c = collision
        .polygons
        .iter()
        .flat_map(move |p| p.transform(transform.translation.current, transform.rotation.current))
        .collect();

    let result = engine::collision::check_collision(&collider, &c, velocity_dir);
    match result {
        Intersection::None => {}
        _ => {
            health.amount -= dmg;
        }
    };

    return false;
}
