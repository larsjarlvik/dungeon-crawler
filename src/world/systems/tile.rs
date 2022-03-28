use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::{BuildChildren, DespawnRecursiveExt};
use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::world::*;

pub fn tile(mut commands: Commands, camera: Res<resources::Camera>, mut query: Query<(Entity, &mut components::Tile)>) {
    let mut rng = StdRng::seed_from_u64(54); // TODO: seed
    let flicker_speed = rng.gen::<f32>() * 0.05 + 0.02;

    for (entity, mut tile) in query.iter_mut() {
        match tile.state {
            components::TileState::Active => {
                if !camera.frustum.test_bounding_box(&tile.bounding_box) {
                    commands.entity(entity).despawn_recursive();
                }
            }

            components::TileState::Destroyed => {
                if camera.frustum.test_bounding_box(&tile.bounding_box) {
                    let mut tile_entity = commands.spawn_bundle((
                        components::Model::new(tile.mesh_id.as_str(), 1.0),
                        components::Render { cull_frustum: true },
                        components::Transform::from_translation_angle(tile.center, tile.rotation),
                    ));
                    let tile_id = tile_entity.id();

                    if tile.collisions.len() > 0 {
                        tile_entity.insert(components::Collision::new(tile.collisions.clone()));
                    }

                    for decor in tile.decor.iter() {
                        let mut decor_entity = commands.spawn_bundle((
                            components::Model::new(decor.mesh_id.as_str(), 1.0),
                            components::Transform::from_translation_angle(decor.position, decor.rotation),
                            components::Render { cull_frustum: true },
                            components::Shadow,
                            components::Health::new(2.0),
                        ));

                        if decor.collisions.len() > 0 {
                            decor_entity.insert(components::Collision::new(decor.collisions.clone()));
                        }

                        let decor_id = decor_entity.id();
                        commands.entity(tile_id).push_children(&[decor_id]);

                        for l in decor.lights.iter() {
                            let mut light_entity = commands.spawn_bundle((
                                components::Light::new(l.color, l.intensity, l.radius, l.offset, l.bloom),
                                components::Render { cull_frustum: true },
                                components::Transform::from_translation_angle(l.position, l.rotation),
                            ));

                            if let Some(flicker) = get_flicker(l.flicker, flicker_speed) {
                                light_entity.insert(flicker);
                            }

                            let light_entity_id = light_entity.id();
                            commands.entity(decor_id).push_children(&[light_entity_id]);
                        }

                        for e in decor.emitters.iter() {
                            let id = e.emitter_id.clone();
                            let mut emitter_entity = commands.spawn_bundle((
                                components::Particle::new(id, e.start_color, e.end_color, e.size, e.strength),
                                components::Render { cull_frustum: true },
                                components::Transform::from_translation_angle(e.position, e.rotation),
                            ));

                            if let Some(flicker) = get_flicker(e.flicker, flicker_speed) {
                                emitter_entity.insert(flicker);
                            }

                            let emitter_entity_id = emitter_entity.id();
                            commands.entity(decor_id).push_children(&[emitter_entity_id]);
                        }
                    }

                    for h in tile.hostiles.iter() {
                        commands.spawn_bundle((
                            components::Model::new(h.mesh_id.as_str(), 1.5),
                            components::Collision::new(h.collider.clone()),
                            components::Animations::new("base", "idle", true),
                            components::Transform::from_translation_scale(h.position, 0.8),
                            components::Render { cull_frustum: true },
                            components::Agressor::new(6.0),
                            components::Movement::new(10.0),
                            components::Shadow,
                            components::Action::new(),
                            components::Health::new(5.0),
                        ));
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
