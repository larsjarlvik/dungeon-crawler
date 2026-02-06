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
    let all_collisions: Vec<(String, Vec<Polygon>)> = collision_query
        .iter()
        .map(|(c, t)| {
            let polygons = c
                .polygons
                .iter()
                .map(|p| p.transform(t.translation.current, t.rotation.current))
                .collect();
            (c.key.clone(), polygons)
        })
        .collect();

    movement_query.par_iter_mut().for_each(|(mut movement, collider, transform)| {
        if movement.velocity == 0.0 {
            return;
        }

        for polygon in collider.polygons.iter() {
            let polygon = polygon.transform(transform.translation.current, transform.rotation.current);

            for (key, collisions) in &all_collisions {
                if *key != collider.key {
                    movement.to = engine::collision::get_collision_offset(movement.to, &polygon, collisions);
                }
            }
        }
    });
}
