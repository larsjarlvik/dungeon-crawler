use crate::world::*;
use bevy_ecs::prelude::*;
use engine::collision::{Polygon, PolygonMethods};

pub fn collision(
    mut movement_query: Query<(
        &mut components::Movement,
        &components::Collision,
        &engine::ecs::components::Transform,
    )>,
    collision_query: Query<(&components::Collision, &engine::ecs::components::Transform)>,
) {
    for (mut movement, collider, transform) in movement_query.iter_mut() {
        if movement.velocity == 0.0 {
            continue;
        }

        let collisions: Vec<Polygon> = collision_query
            .iter()
            .filter(|(c, _)| c.key != collider.key)
            .flat_map(|(c, t)| {
                c.polygons
                    .iter()
                    .map(move |p| p.transform(t.translation.current, t.rotation.current))
            })
            .collect();

        let collider: Vec<Polygon> = collider
            .polygons
            .iter()
            .map(|p| p.transform(transform.translation.current, transform.rotation.current))
            .collect();

        for polygon in collider.iter() {
            movement.to = engine::collision::get_collision_offset(movement.to, &polygon, &collisions);
        }
    }
}
