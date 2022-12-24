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
    movement_query.par_for_each_mut(1, |(mut movement, collider, transform)| {
        if movement.velocity == 0.0 {
            return;
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

        for polygon in collider.polygons.iter() {
            let polygon = polygon.transform(transform.translation.current, transform.rotation.current);
            movement.to = engine::collision::polygon_polygons_offset(movement.to, &polygon, &collisions);
        }
    });
}
