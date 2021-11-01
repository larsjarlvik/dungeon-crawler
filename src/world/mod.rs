use crate::{config, engine, map, utils::Interpolate};
use specs::*;
use std::time::Instant;
pub mod components;
pub mod resources;
pub mod systems;
use cgmath::*;

pub struct World {
    pub components: specs::World,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
    update_time: f32,
    last_frame: std::time::Instant,
    map: map::Map,
    character: engine::model::GltfModel,
}

impl<'a> World {
    pub fn new(engine: &engine::Engine) -> Self {
        let character = engine.load_model("models/character.glb");
        let map = map::Map::new(&engine, 42312, 20);

        let mut components = specs::World::new();
        components.register::<components::Render>();
        components.register::<components::Model>();
        components.register::<components::Transform>();
        components.register::<components::Transform2d>();
        components.register::<components::Text>();
        components.register::<components::Fps>();
        components.register::<components::Light>();
        components.register::<components::Animations>();
        components.register::<components::UserControl>();
        components.register::<components::Movement>();
        components.register::<components::Follow>();
        components.register::<components::Collider>();
        components.register::<components::Collision>();
        components.register::<components::Flicker>();
        components.register::<components::Particle>();
        components.register::<components::Shadow>();

        components.insert(resources::Camera::new(engine.ctx.viewport.get_aspect()));
        components.insert(resources::Input::default());
        components.insert(resources::Time::default());

        let dispatcher = DispatcherBuilder::new()
            .with(systems::Fps, "fps", &[])
            .with(systems::UserControl, "user_control", &[])
            .with(systems::Movement, "movement", &[])
            .with(systems::Flicker, "flicker", &[])
            .build();

        Self {
            components,
            dispatcher,
            update_time: 0.0,
            last_frame: Instant::now(),
            character,
            map,
        }
    }

    pub fn init(&mut self, engine: &engine::Engine) {
        let start = Instant::now();

        self.components.delete_all();

        self.components
            .create_entity()
            .with(components::Fps::new())
            .with(components::Text::new(""))
            .with(components::Transform2d::from_translation_scale(vec2(20.0, 20.0), 18.0))
            .build();

        self.components
            .create_entity()
            .with(components::Model::new(&engine, &self.character, "character"))
            .with(components::Collider::new(&self.character, "character"))
            .with(components::Animations::new("base", "idle"))
            .with(components::Transform::from_translation(vec3(0.0, 0.0, 0.0)))
            .with(components::Light::new(
                vec3(1.0, 1.0, 0.72),
                0.6,
                Some(10.0),
                vec3(0.0, 2.5, 0.0),
            ))
            .with(components::Movement::new(15.0))
            .with(components::UserControl)
            .with(components::Render { cull_frustum: false })
            .with(components::Shadow)
            .with(components::Follow)
            .build();

        self.map.reset();

        if let Some(tile) = &map::edit_mode() {
            self.map.single_tile(&engine, &mut self.components, &tile);
        } else {
            self.map.generate();
        }

        println!("World: {} ms", start.elapsed().as_millis());
    }

    pub fn update(&mut self, engine: &engine::Engine) {
        self.update_time += self.last_frame.elapsed().as_secs_f32();
        self.update_time = self.update_time.min(5.0);

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
            let mut fps = self.components.write_storage::<components::Fps>();
            for fps in (&mut fps).join() {
                fps.fps += 1;
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

        self.map.update(&engine, &mut self.components);
        self.last_frame = Instant::now();
    }
}
