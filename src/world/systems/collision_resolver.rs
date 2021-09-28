use crate::world::*;

pub struct CollisionResolver;

impl<'a> System<'a> for CollisionResolver {
    type SystemData = (
        ReadStorage<'a, components::Collider>,
        WriteStorage<'a, components::Transform>,
    );

    fn run(&mut self, (collider, mut transform): Self::SystemData) {
        for (collider, transform) in (&collider, &mut transform).join() {
            dbg!(collider.is_colliding);
        }
    }
}
