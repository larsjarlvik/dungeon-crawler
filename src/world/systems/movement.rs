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
            .map(|(c, t)| c.polygon.transform(Some(t.translation.current)))
            .collect();

        for (movement, transform, animation, collider) in
            (&mut movement, &mut transform, (&mut animation).maybe(), (&collider).maybe()).join()
        {
            let velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;
            transform.rotation.set(cgmath::Quaternion::from_angle_y(Rad(movement.direction)));

            let mut offset = velocity_dir;

            if let Some(collider) = collider {
                for collision in collisions.iter() {
                    let result = engine::collision::check_collision(
                        &collider.polygon.transform(Some(transform.translation.current)),
                        &collision,
                        vec2(velocity_dir.x, velocity_dir.z),
                    );

                    offset = match result {
                        Intersection::None => offset,
                        Intersection::WillIntersect(mtv) => velocity_dir + vec3(mtv.x, 0.0, mtv.y),
                        Intersection::Intersect => vec3(0.0, 0.0, 0.0),
                    };
                }
            }

            let velocity = vec2(offset.x, offset.z).distance(vec2(0.0, 0.0));

            if let Some(animation) = animation {
                if velocity > 0.01 {
                    transform.translation.set(transform.translation.current + offset);
                    let animation_velocity = velocity / 0.05;
                    if animation_velocity > 1.6 {
                        animation.set_animation("base", "run", animation_velocity);
                    } else {
                        animation.set_animation("base", "walk", animation_velocity);
                    }
                } else {
                    transform.translation.freeze();
                    animation.set_animation("base", "idle", 1.0);
                }
            }

            movement.velocity *= 0.9;
        }
    }
}
