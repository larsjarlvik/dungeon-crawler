use crate::{
    engine::{
        self,
        collision::{Intersection, Polygon, PolygonMethods},
    },
    world::*,
};
use cgmath::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Animations>,
        ReadStorage<'a, components::Collider>,
        ReadStorage<'a, components::Collision>,
        ReadStorage<'a, components::Action>,
    );

    fn run(&mut self, (mut movement, mut transform, mut animation, collider, collision, action): Self::SystemData) {
        let collisions: Vec<Polygon> = (&collision, &transform)
            .join()
            .flat_map(|(c, t)| {
                c.polygons
                    .iter()
                    .map(move |p| p.transform(t.translation.current, t.rotation.current))
            })
            .collect();

        for (movement, transform, animation, action, collider) in
            (&mut movement, &mut transform, &mut animation, &action, (&collider).maybe()).join()
        {
            let mut velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;

            let current_rot = transform.rotation.current;
            let new_rot = cgmath::Quaternion::from_angle_y(Rad(movement.direction));

            transform.rotation.set(current_rot.slerp(new_rot, 0.2));

            if let Some(collider) = collider {
                let collider: Vec<Polygon> = collider
                    .polygons
                    .iter()
                    .map(|p| p.transform(transform.translation.current, transform.rotation.current))
                    .collect();

                for polygon in collider {
                    let collision = get_collision_offset(velocity_dir, &polygon, &collisions);
                    if collision.distance(Vector3::zero()) < velocity_dir.distance(Vector3::zero()) {
                        velocity_dir = collision;
                    }
                }
            }

            match action.current {
                components::CurrentAction::Attack => {
                    animation.set_animation("base", "attack", 2.0);
                    transform.translation.set(transform.translation.current + velocity_dir);
                    movement.velocity *= 0.85;
                }
                components::CurrentAction::None => {
                    let velocity = vec2(velocity_dir.x, velocity_dir.z).distance(Vector2::zero());
                    if velocity > 0.01 {
                        transform.translation.set(transform.translation.current + velocity_dir);

                        let animation_velocity = velocity / 0.04;
                        if animation_velocity > 2.5 {
                            animation.set_animation("base", "run", animation_velocity * 0.4);
                        } else if animation_velocity > 0.3 {
                            animation.set_animation("base", "walk", animation_velocity);
                        }
                    } else {
                        transform.translation.freeze();
                        animation.set_animation("base", "idle", 1.0);
                    }

                    movement.velocity *= 0.9;
                }
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
