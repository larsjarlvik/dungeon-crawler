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

        let mut velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;

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
            let collision = get_collision_offset(velocity_dir, &polygon, &collisions);
            velocity_dir = collision;
        }

        movement.velocity_dir = velocity_dir;
    }
}

fn get_collision_offset(velocity_dir: Vector3<f32>, collider: &Polygon, collisions: &Vec<Polygon>) -> Vector3<f32> {
    let mut offset = velocity_dir;
    let mut hits = 0;

    for collision in collisions.iter() {
        if collision.center().distance(collider.center()) > 3.0 {
            continue;
        }

        let result = engine::collision::check_collision(&collider, &collision, vec2(velocity_dir.x, velocity_dir.z));
        offset = match result {
            Intersection::WillIntersect(mtv) => {
                hits += 1;
                offset + velocity_dir + vec3(mtv.x, 0.0, mtv.y)
            }
            _ => offset,
        };
    }

    offset / (hits + 1) as f32
}
