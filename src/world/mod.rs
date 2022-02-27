use crate::{
    config,
    engine::{self, Context},
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
    pub character: engine::model::GltfModel,
}

pub struct World {
    pub components: bevy_ecs::world::World,
    pub schedule: Schedule,
    pub post_schedule: Schedule,
    pub game_state: GameState,
    update_time: f32,
    last_frame: std::time::Instant,
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
                .with_system(systems::user_control.label("user_control"))
                .with_system(systems::movement.after("user_control")),
        );

        let mut post_schedule = Schedule::default();
        post_schedule.add_stage(
            "post",
            SystemStage::single_threaded()
                .with_system(systems::camera)
                .with_system(systems::animation),
        );

        Self {
            components,
            schedule,
            post_schedule,
            update_time: 0.0,
            last_frame: Instant::now(),
            resources: None,
            game_state: GameState::Loading,
        }
    }

    pub fn load_resources(&mut self, engine: &engine::Engine) {
        let start = Instant::now();
        self.resources = Some(Resources {
            character: engine.load_model("models/character.glb"),
            map: map::Map::new(&engine, 42312, 3),
        });
        println!("Load resources {} ms", start.elapsed().as_millis());
    }

    pub fn init(&mut self, engine: &engine::Engine) {
        let mut components = create_components(&engine.ctx);

        if let Some(resources) = &mut self.resources {
            components
                .spawn()
                .insert(components::Model::new(&engine, &resources.character, "character"))
                .insert(components::Collider::new(&resources.character, "character"))
                .insert(components::Animations::new("base", "idle"))
                .insert(components::Transform::from_translation_scale(vec3(0.0, 0.0, 0.0), 0.01))
                .insert(components::Light::new(
                    vec3(1.0, 1.0, 0.72),
                    0.6,
                    Some(10.0),
                    vec3(0.0, 2.5, 0.0),
                    0.0,
                ))
                .insert(components::Movement::new(15.0))
                .insert(components::Action::new())
                .insert(components::UserControl)
                .insert(components::Render { cull_frustum: false })
                .insert(components::Shadow)
                .insert(components::Follow);

            resources.map.reset();

            if let Some(tile) = &map::edit_mode() {
                resources.map.single_tile(&engine, &mut components, &tile);
            } else {
                resources.map.generate();
            }
        }

        self.components = components;
    }

    pub fn update(&mut self, engine: &engine::Engine) {
        self.update_time += self.last_frame.elapsed().as_secs_f32();
        self.update_time = self.update_time.min(5.0);

        if let Some(resources) = &mut self.resources {
            {
                let mut fps = self.components.get_resource_mut::<resources::Fps>().unwrap();
                fps.update();
            }

            match self.game_state {
                GameState::Running => {
                    while self.update_time >= 0.0 {
                        self.schedule.run(&mut self.components);
                        self.update_time -= config::time_step().as_secs_f32();

                        {
                            let mut time = self.components.get_resource_mut::<resources::Time>().unwrap();
                            time.reset();
                        }
                    }

                    let mut time = self.components.get_resource_mut::<resources::Time>().unwrap();
                    time.freeze(self.last_frame.elapsed().as_secs_f32());

                    self.post_schedule.run(&mut self.components);
                }
                _ => {
                    self.update_time = 0.0;
                }
            }

            resources.map.update(&engine, &mut self.components);
        }
        self.last_frame = Instant::now();
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
