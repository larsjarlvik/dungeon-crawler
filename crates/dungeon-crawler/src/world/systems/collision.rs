use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;
use engine::collision::{Intersection, Polygon, PolygonMethods};

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
            let closest_target = get_collision_offset(movement.to, &polygon, &collisions);
            movement.to = closest_target;
        }
    }
}

fn get_collision_offset(position: Vector3<f32>, collider: &Polygon, collisions: &Vec<Polygon>) -> Vector3<f32> {
    let mut offset = position;
    let mut hits = 0;

    for collision in collisions.iter() {
        if collision.center().distance(collider.center()) > 3.0 {
            continue;
        }

        let result = engine::collision::check_collision(&collider, &collision, vec2(position.x, position.z));
        offset = match result {
            Intersection::WillIntersect(mtv) => {
                hits += 1;
                offset + position + vec3(mtv.x, 0.0, mtv.y)
            }
            _ => offset,
        };
    }

    offset / (hits + 1) as f32
}
