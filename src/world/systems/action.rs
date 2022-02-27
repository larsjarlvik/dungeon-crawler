use crate::{
    engine::collision::{Intersection, Polygon, PolygonMethods},
    world::*,
};
use cgmath::*;

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteStorage<'a, components::Action>,
        WriteStorage<'a, components::Health>,
        ReadStorage<'a, components::Movement>,
        ReadStorage<'a, components::Collider>,
        ReadStorage<'a, components::Collision>,
        ReadStorage<'a, components::Transform>,
    );

    fn run(&mut self, (entities, lazy, mut action, mut health, movement, collider, collision, transform): Self::SystemData) {
        for (action, movement, collider, t) in (&mut action, &movement, &collider, &transform).join() {
            match action.current {
                components::CurrentAction::Attack(dmg) => {
                    let collider: Vec<Polygon> = collider
                        .polygons
                        .iter()
                        .map(|p| p.transform(t.translation.current, t.rotation.current))
                        .collect();

                    let velocity_dir = vec2(movement.direction.sin(), movement.direction.cos());
                    for polygon in collider {
                        for (entity, health, collision, transform) in (&entities, &mut health, &collision, &transform).join() {
                            if health.amount > 0.0 {
                                attack(&polygon, health, collision, transform, velocity_dir, dmg);
                                if health.amount <= 0.0 {
                                    lazy.insert(entity, components::Delete);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }

            if let Some(action_changed) = action.set {
                if action_changed.elapsed().as_secs_f32() > action.length {
                    action.reset();
                }
            }
        }
    }
}

fn attack(
    collider: &Polygon,
    health: &mut components::Health,
    collision: &components::Collision,
    transform: &components::Transform,
    velocity_dir: Vector2<f32>,
    dmg: f32,
) -> bool {
    let c = collision
        .polygons
        .iter()
        .flat_map(move |p| p.transform(transform.translation.current, transform.rotation.current))
        .collect();

    let result = engine::collision::check_collision(&collider, &c, velocity_dir);
    match result {
        Intersection::None => {}
        _ => {
            health.amount -= dmg;
        }
    };

    return false;
}
