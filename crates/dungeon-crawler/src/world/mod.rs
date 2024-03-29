use crate::{config, map};
use std::time::Instant;
pub mod components;
pub mod resources;
pub mod systems;
use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum GameState {
    Loading,
    Running,
    MainMenu,
    Terminated,
    Reload,
}

pub struct Resources {
    pub map: map::Map,
    pub character: engine::model::GltfModel,
}

pub struct World {
    pub components: bevy_ecs::world::World,
    pub schedule: Schedule,
    pub post_schedule: Schedule,
    pub game_state: GameState,
    pub resources: Option<Resources>,
}

impl World {
    pub fn new(engine: &engine::Engine) -> Self {
        let components = setup_world(&engine.ctx);

        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(systems::flicker)
                .with_system(systems::user_control)
                .with_system(systems::actions.label("actions"))
                .with_system(systems::collision.label("collision").after("actions"))
                .with_system(systems::damage.after("actions"))
                .with_system(systems::movement.after("collision"))
                .with_system(systems::aggression)
                .with_system(systems::health),
        );

        let mut post_schedule = Schedule::default();
        post_schedule.add_stage(
            "post",
            SystemStage::parallel()
                .with_system(systems::tile)
                .with_system(engine::ecs::systems::camera)
                .with_system(engine::ecs::systems::animation.label("animation"))
                .with_system(engine::ecs::systems::player.after("animation")),
        );

        Self {
            components,
            schedule,
            post_schedule,
            resources: None,
            game_state: GameState::Loading,
        }
    }

    pub fn init(&mut self, engine: &mut engine::Engine) {
        self.components.clear_entities();

        if let Some(resources) = &mut self.resources {
            let character_model = engine.initialize_model(&resources.character, "character");
            let collider = resources
                .character
                .collisions
                .get("character")
                .expect("Could not find character collider!");

            self.components.spawn((
                engine::ecs::components::Animations::new("base", "idle", engine::ecs::components::AnimationStatus::Repeat),
                character_model,
                components::Collision::new(collider.clone()),
                engine::ecs::components::Transform::from_translation_scale(vec3(0.0, 0.0, 0.0), 0.01),
                components::Movement::new(15.0),
                components::ActionExecutor::new(),
                components::Stats::new(15, 15, 15, 0, config::TEAM_FRIENDLY),
                components::Weapon {
                    damage: 2.0..7.0,
                    distance: 0.5,
                    radius: 0.25,
                    time: 1.0,
                },
                components::UserControl::default(),
                engine::ecs::components::SoundEffects::default(),
                engine::ecs::components::Render { cull_frustum: false },
                engine::ecs::components::Shadow,
                engine::ecs::components::Follow,
                engine::ecs::components::Light::new(vec3(1.0, 0.94, 0.88), 0.5, 7.0, vec3(0.0, 3.0, 0.0), 0.0),
                components::Target,
            ));

            if let Some((tile_name, variant)) = map::edit_mode() {
                resources.map.single_tile(engine, &mut self.components, &tile_name, variant);
            } else {
                resources.map.generate(&mut self.components, engine);
            }
        }
    }

    pub fn reset_time(&mut self) {
        let mut time = self.components.get_resource_mut::<engine::ecs::resources::Time>().unwrap();
        time.accumulator = 0.0;
        time.time = Instant::now();
    }

    pub fn update(&mut self) {
        let time_step = config::time_step().as_secs_f32();
        let mut accumulator = {
            let mut time = self.components.get_resource_mut::<engine::ecs::resources::Time>().unwrap();
            time.accumulator += (Instant::now() - time.time).as_secs_f32();
            time.accumulator.min(3.0)
        };

        {
            let mut fps = self.components.get_resource_mut::<resources::Fps>().unwrap();
            fps.update();
        }

        if self.game_state == GameState::Running {
            while accumulator >= time_step {
                let mut time = self.components.get_resource_mut::<engine::ecs::resources::Time>().unwrap();
                time.frame += 1;
                accumulator -= time_step;

                self.schedule.run(&mut self.components);
            }

            let mut time = self.components.get_resource_mut::<engine::ecs::resources::Time>().unwrap();
            time.freeze(accumulator, time_step);
            self.post_schedule.run(&mut self.components);
        } else {
            let mut time = self.components.get_resource_mut::<engine::ecs::resources::Time>().unwrap();
            time.freeze(0.0, time_step);
        }
    }

    pub fn is_dead(&mut self) -> bool {
        let stats = self
            .components
            .query_filtered::<&components::Stats, With<components::UserControl>>()
            .get_single(&self.components)
            .expect("No character stats found!");

        stats.health.get() <= 0.0 && stats.health.last_change.elapsed().as_secs_f32() > 3.0
    }

    pub fn load_resources(&mut self, ctx: &engine::Context) {
        let start = Instant::now();
        let character = engine::load_model(ctx, "models/character.glb");
        let map = map::Map::new(ctx, 42312, 3);

        let mut sound_effects = self
            .components
            .get_non_send_resource_mut::<engine::ecs::resources::SoundEffects>()
            .unwrap();

        sound_effects.load(&character.get_sound_effects());
        sound_effects.load(&map.sound_effects);
        sound_effects.volume = ctx.settings.audio_effects;

        self.set_sounds(ctx);

        println!("Load resources {} ms", start.elapsed().as_millis());
        self.resources = Some(Resources { map, character });
    }

    pub fn set_sounds(&mut self, ctx: &engine::Context) {
        {
            let mut sound_ambience = self
                .components
                .get_non_send_resource_mut::<engine::ecs::resources::SoundAmbience>()
                .unwrap();

            sound_ambience.volume = ctx.settings.audio_ambient;
            sound_ambience.play("ambience");
        }

        {
            let mut sound_effects = self
                .components
                .get_non_send_resource_mut::<engine::ecs::resources::SoundEffects>()
                .unwrap();

            sound_effects.volume = ctx.settings.audio_effects;
        }
    }
}

pub fn setup_world(ctx: &engine::Context) -> bevy_ecs::world::World {
    let mut components = bevy_ecs::world::World::new();

    components.insert_resource(engine::ecs::resources::Camera::new(ctx.viewport.get_aspect()));
    components.insert_resource(engine::ecs::resources::Time::default());
    components.insert_non_send_resource(engine::ecs::resources::SoundEffects::default());
    components.insert_non_send_resource(engine::ecs::resources::SoundAmbience::default());
    components.insert_resource(engine::ecs::resources::Input::default());
    components.insert_resource(resources::Fps::default());

    components
}
