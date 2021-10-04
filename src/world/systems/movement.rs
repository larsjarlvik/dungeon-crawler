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
    );

    fn run(&mut self, (mut movement, mut transform, mut animation, collider, collision): Self::SystemData) {
        let collisions: Vec<Polygon> = (&collision, &transform)
            .join()
            .flat_map(|(c, t)| c.polygon.iter().map(move |p| p.transform(Some(t.translation.current))))
            .collect();

        for (movement, transform, animation, collider) in
            (&mut movement, &mut transform, (&mut animation).maybe(), (&collider).maybe()).join()
        {
            let mut velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;
            transform.rotation.set(cgmath::Quaternion::from_angle_y(Rad(movement.direction)));

            if let Some(collider) = collider {
                let collider: Vec<Polygon> = collider
                    .polygon
                    .iter()
                    .map(|p| p.transform(Some(transform.translation.current.clone())))
                    .collect();
                for polygon in collider {
                    velocity_dir = get_collision_offset(velocity_dir, &polygon, &collisions);
                }
            }

            let velocity = vec2(velocity_dir.x, velocity_dir.z).distance(vec2(0.0, 0.0));
            if velocity > 0.01 {
                transform.translation.set(transform.translation.current + velocity_dir);

                if let Some(animation) = animation {
                    let animation_velocity = velocity / 0.05;
                    if animation_velocity > 1.6 {
                        animation.set_animation("base", "run", animation_velocity);
                    } else if animation_velocity > 0.3 {
                        animation.set_animation("base", "walk", animation_velocity);
                    }
                }
            } else if let Some(animation) = animation {
                transform.translation.freeze();
                animation.set_animation("base", "idle", 1.0);
            }

            movement.velocity *= 0.9;
        }
    }
}

fn get_collision_offset(velocity_dir: Vector3<f32>, collider: &Polygon, collisions: &Vec<Polygon>) -> Vector3<f32> {
    let mut offset = velocity_dir;
    let mut hits = 0;

    for collision in collisions.iter() {
        let result = engine::collision::check_collision(&collider.transform(Some(offset)), &collision, vec2(offset.x, offset.z));

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
