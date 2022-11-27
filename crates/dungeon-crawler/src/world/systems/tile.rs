use bevy_ecs::prelude::*;
use bevy_hierarchy::*;
use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::world::*;

pub fn tile(mut commands: Commands, camera: Res<engine::ecs::resources::Camera>, mut query: Query<(Entity, &mut components::Tile)>) {
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
                    let mut tile_entity = commands.spawn((
                        tile.model.clone(),
                        engine::ecs::components::Render { cull_frustum: true },
                        engine::ecs::components::Transform::from_translation_angle(tile.center, tile.rotation),
                    ));
                    let tile_id = tile_entity.id();

                    if !tile.collisions.is_empty() {
                        tile_entity.insert(components::Collision::new(tile.collisions.clone()));
                    }

                    for decor in tile.decor.iter() {
                        let mut decor_entity = commands.spawn((
                            decor.model.clone(),
                            engine::ecs::components::Transform::from_translation_angle(decor.position, decor.rotation),
                            engine::ecs::components::Render { cull_frustum: true },
                            engine::ecs::components::Shadow,
                        ));

                        if !decor.collisions.is_empty() {
                            decor_entity.insert(components::Collision::new(decor.collisions.clone()));
                        }

                        let decor_id = decor_entity.id();
                        commands.entity(tile_id).push_children(&[decor_id]);

                        for l in decor.lights.iter() {
                            let mut light_entity = commands.spawn((
                                engine::ecs::components::Light::new(l.color, l.intensity, l.radius, l.offset, l.bloom),
                                engine::ecs::components::Render { cull_frustum: true },
                                engine::ecs::components::Transform::from_translation_angle(l.position, l.rotation),
                            ));

                            if let Some(flicker) = get_flicker(l.flicker, flicker_speed) {
                                light_entity.insert(flicker);
                            }

                            let light_entity_id = light_entity.id();
                            commands.entity(decor_id).push_children(&[light_entity_id]);
                        }

                        for e in decor.emitters.iter() {
                            let id = e.emitter_id.clone();
                            let mut emitter_entity = commands.spawn((
                                engine::ecs::components::Particle::new(id, e.start_color, e.end_color, e.size, e.strength),
                                engine::ecs::components::Render { cull_frustum: true },
                                engine::ecs::components::Transform::from_translation_angle(e.position, e.rotation),
                            ));

                            if let Some(flicker) = get_flicker(e.flicker, flicker_speed) {
                                emitter_entity.insert(flicker);
                            }

                            let emitter_entity_id = emitter_entity.id();
                            commands.entity(decor_id).push_children(&[emitter_entity_id]);
                        }
                    }

                    for hostile in tile.hostiles.iter() {
                        commands.spawn((
                            components::Name::new("Skeleton Warrior"),
                            hostile.model.clone(),
                            components::Collision::new(hostile.collider.clone()),
                            engine::ecs::components::Animations::new("base", "idle", engine::ecs::components::AnimationStatus::Repeat),
                            engine::ecs::components::Transform::from_translation_scale(hostile.position, 0.8),
                            engine::ecs::components::Render { cull_frustum: true },
                            engine::ecs::components::SoundEffects::default(),
                            components::Stats::new(10, 10, 12, components::stats::get_level_experience(3)),
                            components::Weapon {
                                damage: 2.0..5.0,
                                time: 1.0,
                            },
                            components::Agressor::new(6.0),
                            components::Movement::new(10.0),
                            engine::ecs::components::Shadow,
                            components::ActionExecutor::new(),
                        ));
                    }

                    tile.state = components::TileState::Active;
                }
            }
        }
    }
}

fn get_flicker(flicker: Option<f32>, speed: f32) -> Option<components::Flicker> {
    flicker.map(|flicker| components::Flicker::new(flicker, speed))
}
