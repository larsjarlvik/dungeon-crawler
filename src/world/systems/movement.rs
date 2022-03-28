use crate::{
    engine::{
        self,
        collision::{Intersection, Polygon, PolygonMethods},
    },
    world::*,
};
use bevy_ecs::prelude::*;
use cgmath::*;

pub fn movement(
    time: Res<resources::Time>,
    mut query: QuerySet<(
        QueryState<(&components::Collision, &components::Transform)>,
        QueryState<(
            &mut components::Movement,
            &mut components::Transform,
            &mut components::Animations,
            &components::Action,
            Option<&components::Collider>,
        )>,
    )>,
) {
    let collisions: Vec<Polygon> = query
        .q0()
        .iter()
        .flat_map(|(c, t)| {
            c.polygons
                .iter()
                .map(move |p| p.transform(t.translation.current, t.rotation.current))
        })
        .collect();

    for (mut movement, mut transform, mut animation, action, collider) in query.q1().iter_mut() {
        let mut velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;

        let current_rot = transform.rotation.current;
        let current_trans = transform.translation.current;

        let new_rot = cgmath::Quaternion::from_angle_y(Rad(movement.direction));

        transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);

        if let Some(collider) = collider {
            let collider: Vec<Polygon> = collider
                .polygons
                .iter()
                .map(|p| p.transform(current_trans, transform.rotation.current))
                .collect();

            for polygon in collider {
                let collision = get_collision_offset(velocity_dir, &polygon, &collisions);
                if collision.distance(Vector3::zero()) < velocity_dir.distance(Vector3::zero()) {
                    velocity_dir = collision;
                }
            }
        }

        match action.current {
            components::CurrentAction::Attack(_) => {
                animation.set_animation("base", "attack", 2.0, false);
                transform.translation.set(current_trans + velocity_dir, time.frame);
                movement.velocity *= 0.85;
            }
            components::CurrentAction::None => {
                let velocity = vec2(velocity_dir.x, velocity_dir.z).distance(Vector2::zero());
                if velocity > 0.01 {
                    transform.translation.set(current_trans + velocity_dir, time.frame);

                    let animation_velocity = velocity / 0.04;
                    if animation_velocity > 2.5 {
                        animation.set_animation("base", "run", animation_velocity * 0.4, true);
                    } else if animation_velocity > 0.3 {
                        animation.set_animation("base", "walk", animation_velocity, true);
                    }
                } else {
                    transform.translation.freeze();
                    animation.set_animation("base", "idle", 1.0, true);
                }

                movement.velocity *= 0.9;
            }
            components::CurrentAction::Hit => {
                animation.set_animation("base", "hit", 2.0, false);
            }
            components::CurrentAction::Death => {
                animation.set_animation("base", "death", 2.0, false);
            }
        }
    }
}

fn get_collision_offset(velocity_dir: Vector3<f32>, collider: &Polygon, collisions: &Vec<Polygon>) -> Vector3<f32> {
    let mut offset = velocity_dir;
    let mut hits = 0;

    for collision in collisions.iter() {
        if collision.center().distance(collider.center()) > 5.0 {
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
