use crate::{
    config,
    engine::{self, Context},
    map,
    utils::Interpolate,
};
use specs::*;
use std::time::Instant;
pub mod components;
pub mod resources;
pub mod systems;
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
    pub components: specs::World,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
    pub game_state: GameState,
    update_time: f32,
    last_frame: std::time::Instant,
    pub resources: Option<Resources>,
}

impl<'a> World {
    pub fn new(engine: &engine::Engine) -> Self {
        let components = create_components(&engine.ctx);
        let dispatcher = DispatcherBuilder::new()
            .with(systems::Action, "action", &[])
            .with(systems::Parent, "parent", &["action"])
            .with(systems::UserControl, "user_control", &["action"])
            .with(systems::Movement, "movement", &[])
            .with(systems::Flicker, "flicker", &[])
            .build();

        Self {
            components,
            dispatcher,
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
                .create_entity()
                .with(components::Model::new(&engine, &resources.character, "character"))
                .with(components::Collider::new(&resources.character, "character"))
                .with(components::Animations::new("base", "idle"))
                .with(components::Transform::from_translation_scale(vec3(0.0, 0.0, 0.0), 0.01))
                .with(components::Light::new(
                    vec3(1.0, 1.0, 0.72),
                    0.6,
                    Some(10.0),
                    vec3(0.0, 2.5, 0.0),
                    0.0,
                ))
                .with(components::Movement::new(15.0))
                .with(components::Action::new())
                .with(components::UserControl)
                .with(components::Render { cull_frustum: false })
                .with(components::Shadow)
                .with(components::Follow)
                .build();

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
                let mut fps = self.components.write_resource::<resources::Fps>();
                fps.update();
            }

            match self.game_state {
                GameState::Running => {
                    while self.update_time >= 0.0 {
                        self.dispatcher.setup(&mut self.components);
                        self.dispatcher.dispatch_par(&mut self.components);
                        self.components.maintain();
                        self.update_time -= config::time_step().as_secs_f32();

                        {
                            let mut time = self.components.write_resource::<resources::Time>();
                            time.reset();
                        }
                    }

                    {
                        let mut camera = self.components.write_resource::<resources::Camera>();
                        let mut time = self.components.write_resource::<resources::Time>();
                        time.freeze();

                        let follow = self.components.read_storage::<components::Follow>();
                        let transform = self.components.read_storage::<components::Transform>();

                        for (transform, _) in (&transform, &follow).join() {
                            camera.set(transform.translation.get(time.last_frame));
                        }

                        let mut animations = self.components.write_storage::<components::Animations>();
                        for animation in (&mut animations).join() {
                            for (_, channel) in animation.channels.iter_mut() {
                                channel.current.elapsed += self.last_frame.elapsed().as_secs_f32() * channel.current.speed;
                                if let Some(previous) = &mut channel.prev {
                                    previous.elapsed += self.last_frame.elapsed().as_secs_f32() * previous.speed;
                                }
                            }
                        }
                    }
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

pub fn create_components(ctx: &Context) -> specs::World {
    let mut components = specs::World::new();
    components.register::<components::Render>();
    components.register::<components::Model>();
    components.register::<components::Transform>();
    components.register::<components::Text>();
    components.register::<components::Light>();
    components.register::<components::Animations>();
    components.register::<components::UserControl>();
    components.register::<components::Movement>();
    components.register::<components::Action>();
    components.register::<components::Follow>();
    components.register::<components::Collider>();
    components.register::<components::Collision>();
    components.register::<components::Flicker>();
    components.register::<components::Particle>();
    components.register::<components::Shadow>();
    components.register::<components::Health>();
    components.register::<components::Child>();
    components.register::<components::Delete>();

    components.insert(resources::Camera::new(ctx.viewport.get_aspect()));
    components.insert(resources::Input::default());
    components.insert(resources::Time::default());
    components.insert(resources::Fps::default());

    components
}
