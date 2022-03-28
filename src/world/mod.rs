use crate::{
    config,
    engine::{self, model, Context},
    map,
    utils::Interpolate,
};
use std::time::Instant;
pub mod components;
pub mod resources;
pub mod systems;
use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
    Loading,
    Running,
    MainMenu,
    Terminated,
    Reload,
}

pub struct Resources {
    pub map: map::Map,
    pub character: model::GltfModel,
}

pub struct World {
    pub components: bevy_ecs::world::World,
    pub schedule: Schedule,
    pub post_schedule: Schedule,
    pub game_state: GameState,
    pub resources: Option<Resources>,
}

impl<'a> World {
    pub fn new(engine: &engine::Engine) -> Self {
        let components = create_components(&engine.ctx);

        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(systems::action)
                .with_system(systems::flicker)
                .with_system(systems::user_control)
                .with_system(systems::movement)
                .with_system(systems::aggression),
        );

        let mut post_schedule = Schedule::default();
        post_schedule.add_stage(
            "post",
            SystemStage::parallel()
                .with_system(systems::tile)
                .with_system(systems::camera)
                .with_system(systems::animation),
        );

        Self {
            components,
            schedule,
            post_schedule,
            resources: None,
            game_state: GameState::Loading,
        }
    }

    pub fn load_resources(&mut self, engine: &mut engine::Engine) {
        let start = Instant::now();
        let character = engine.load_model("models/character.glb");
        engine.initialize_model(&character, "character", "character".to_string());

        self.resources = Some(Resources {
            map: map::Map::new(engine, 42312, 3),
            character,
        });
        println!("Load resources {} ms", start.elapsed().as_millis());
    }

    pub fn init(&mut self, engine: &mut engine::Engine) {
        self.components.clear_entities();

        if let Some(resources) = &mut self.resources {
            let collider = resources
                .character
                .collisions
                .get("character")
                .expect("Could not find character collider!");

            self.components.spawn().insert_bundle((
                components::Model::new("character", 1.5),
                components::Collider::new(collider.clone()),
                components::Animations::new("base", "idle", true),
                components::Transform::from_translation_scale(vec3(0.0, 0.0, 0.0), 0.01),
                components::Movement::new(15.0),
                components::Action::new(),
                components::UserControl,
                components::Render { cull_frustum: false },
                components::Shadow,
                components::Follow,
                components::Target,
                components::Health::new(10.0),
            ));

            if let Some(tile) = &map::edit_mode() {
                resources.map.single_tile(engine, &mut self.components, &tile);
            } else {
                resources.map.generate(&mut self.components, engine);
            }
        }
    }

    pub fn update(&mut self) {
        let time_step = config::time_step().as_secs_f32();
        let mut accumulator = {
            let mut time = self.components.get_resource_mut::<resources::Time>().unwrap();
            time.accumulator += (Instant::now() - time.time).as_secs_f32();
            time.accumulator.min(3.0)
        };

        {
            let mut fps = self.components.get_resource_mut::<resources::Fps>().unwrap();
            fps.update();
        }

        match self.game_state {
            GameState::Running => {
                while accumulator >= time_step {
                    let mut time = self.components.get_resource_mut::<resources::Time>().unwrap();
                    time.frame += 1;
                    accumulator -= time_step;

                    self.schedule.run(&mut self.components);
                }

                let mut time = self.components.get_resource_mut::<resources::Time>().unwrap();
                time.freeze(accumulator, time_step);

                self.post_schedule.run(&mut self.components);
            }
            _ => {}
        }
    }
}

pub fn create_components(ctx: &Context) -> bevy_ecs::world::World {
    let mut components = bevy_ecs::world::World::new();

    components.insert_resource(resources::Camera::new(ctx.viewport.get_aspect()));
    components.insert_resource(resources::Input::default());
    components.insert_resource(resources::Time::default());
    components.insert_resource(resources::Fps::default());

    components
}
