use bevy_ecs::prelude::*;
use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::world::*;

pub fn tile(mut commands: Commands, camera: Res<resources::Camera>, mut query: Query<(Entity, &mut components::Tile)>) {
    let mut rng = StdRng::seed_from_u64(54); // TODO: seed
    let flicker_speed = rng.gen::<f32>() * 0.05 + 0.02;

    for (entity, mut tile) in query.iter_mut() {
        match tile.state {
            components::TileState::Active => {
                if !camera.frustum.test_bounding_box(&tile.bounding_box) {
                    commands.entity(entity).despawn();
                }
            }

            components::TileState::Destroyed => {
                if camera.frustum.test_bounding_box(&tile.bounding_box) {
                    let mut tile_entity = commands.spawn();
                    tile_entity
                        .insert(components::Model::new(tile.mesh_id.as_str()))
                        .insert(components::Render { cull_frustum: true })
                        .insert(components::Transform::from_translation_angle(tile.center, tile.rotation));

                    if tile.collisions.len() > 0 {
                        tile_entity.insert(components::Collision::new(tile.collisions.clone()));
                    }

                    for decor in tile.decor.iter() {
                        let mut decor_entity = commands.spawn();
                        decor_entity
                            .insert(components::Model::new(decor.mesh_id.as_str()))
                            .insert(components::Transform::from_translation_angle(decor.position, decor.rotation))
                            .insert(components::Render { cull_frustum: true })
                            .insert(components::Shadow)
                            .insert(components::Health::new(10.0));

                        if decor.collisions.len() > 0 {
                            decor_entity.insert(components::Collision::new(decor.collisions.clone()));
                        }

                        for l in decor.lights.iter() {
                            let mut light_entity = commands.spawn();
                            light_entity
                                .insert(components::Light::new(l.color, l.intensity, l.radius, l.offset, l.bloom))
                                .insert(components::Render { cull_frustum: true })
                                .insert(components::Transform::from_translation_angle(l.position, l.rotation));

                            if let Some(flicker) = get_flicker(l.flicker, flicker_speed) {
                                light_entity.insert(flicker);
                            }
                        }

                        for e in decor.emitters.iter() {
                            let mut emitter_entity = commands.spawn();
                            let id = e.emitter_id.clone();
                            emitter_entity
                                .insert(components::Particle::new(id, e.start_color, e.end_color, e.size, e.strength))
                                .insert(components::Render { cull_frustum: true })
                                .insert(components::Transform::from_translation_angle(e.position, e.rotation));

                            if let Some(flicker) = get_flicker(e.flicker, flicker_speed) {
                                emitter_entity.insert(flicker);
                            }
                        }
                    }

                    tile.state = components::TileState::Active;
                }
            }
        }
    }
}

fn get_flicker(flicker: Option<f32>, speed: f32) -> Option<components::Flicker> {
    if let Some(flicker) = flicker {
        Some(components::Flicker::new(flicker, speed))
    } else {
        None
    }
}
