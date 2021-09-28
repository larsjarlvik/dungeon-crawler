use crate::{engine, world::*};

pub struct CollisionDetection;

impl<'a> System<'a> for CollisionDetection {
    type SystemData = (
        WriteStorage<'a, components::Collider>,
        ReadStorage<'a, components::Collision>,
        ReadStorage<'a, components::Transform>,
    );

    fn run(&mut self, (mut collider, collision, transform): Self::SystemData) {
        for (collider, collider_transform) in (&mut collider, (&transform).maybe()).join() {
            let collider_transformed = if let Some(transform) = collider_transform {
                collider.transform(transform.translation.current).polygon
            } else {
                collider.polygon.clone()
            };

            let mut is_colliding = false;
            for (collision, collision_transform) in (&collision, (&transform).maybe()).join() {
                let collision_transformed = if let Some(transform) = collision_transform {
                    let transformed = collision.transform(transform.translation.current);
                    transformed.polygon
                } else {
                    collision.polygon.clone()
                };

                if engine::collision::has_collided(&collider_transformed, &collision_transformed, &None) {
                    is_colliding = true;
                    break;
                }
            }

            collider.is_colliding = is_colliding;
        }
    }
}
