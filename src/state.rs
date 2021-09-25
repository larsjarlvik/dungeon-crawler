use std::time::Instant;

use crate::{engine, world};
use cgmath::*;
use specs::{Builder, WorldExt};
use winit::{event::VirtualKeyCode, window::Window};

pub struct State {
    engine: engine::Engine,
    world: world::World,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let engine = engine::Engine::new(window).await;
        let world = world::World::new();

        let mut state = Self { engine, world };
        state.init();
        state
    }

    pub fn init(&mut self) {
        let start = Instant::now();
        let room = self.engine.load_model("models/room.glb");
        let character = self.engine.load_model("models/character.glb");

        self.engine.init();
        self.world = world::World::new();

        self.world
            .components
            .insert(world::resources::Camera::new(self.engine.ctx.viewport.get_aspect()));
        self.world.components.insert(world::resources::Input::default());
        self.world.components.insert(world::resources::Time::default());

        self.world
            .components
            .create_entity()
            .with(world::components::Fps::new())
            .with(world::components::Text::new("", vec2(20.0, 20.0)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.75),
                intensity: 0.4,
                radius: Some(7.1),
            })
            .with(world::components::Transform::from_translation(vec3(-2.4, 2.0, -2.4)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.7),
                intensity: 0.4,
                radius: Some(7.0),
            })
            .with(world::components::Transform::from_translation(vec3(2.4, 2.0, -2.4)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.63),
                intensity: 0.4,
                radius: Some(6.8),
            })
            .with(world::components::Transform::from_translation(vec3(-2.4, 2.0, 2.4)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.72),
                intensity: 0.4,
                radius: Some(7.3),
            })
            .with(world::components::Transform::from_translation(vec3(2.4, 2.0, 2.4)))
            .build();

        for z in -3..3 {
            for x in -3..3 {
                self.world
                    .components
                    .create_entity()
                    .with(world::components::Model::new(&self.engine, &room, "room"))
                    .with(world::components::Transform::from_translation(vec3(
                        x as f32 * 10.0,
                        0.0,
                        z as f32 * 10.0,
                    )))
                    .with(world::components::Render::default())
                    .build();
            }
        }

        self.world
            .components
            .create_entity()
            .with(world::components::Model::new(&self.engine, &character, "character"))
            .with(world::components::Animations::new("base", "idle"))
            .with(world::components::Transform::from_translation(vec3(0.0, 1.0, 0.0)))
            .with(world::components::Movement::new(3.0))
            .with(world::components::UserControl)
            .with(world::components::Render)
            .with(world::components::Follow)
            .build();

        println!("Initialized world in: {} ms", start.elapsed().as_millis());
    }

    pub fn resize(&mut self, window: &Window, active: bool) {
        if active {
            self.engine.set_viewport(window);
            self.engine.deferred_pipeline.resize(&self.engine.ctx);

            let mut camera = self.world.components.write_resource::<world::resources::Camera>();
            *camera = world::resources::Camera::new(self.engine.ctx.viewport.get_aspect());
        } else {
            self.engine.ctx.surface = None;
        }
    }

    #[allow(unused)]
    pub fn keyboard(&mut self, keyboard_input: &winit::event::KeyboardInput) {
        if keyboard_input.virtual_keycode == Some(VirtualKeyCode::R) {
            self.init();
        }

        let mut input = self.world.components.write_resource::<world::resources::Input>();
        input.keyboard(keyboard_input);
    }

    pub fn mouse_move(&mut self, id: u64, x: f32, y: f32) {
        let mut input = self.world.components.write_resource::<world::resources::Input>();
        input.mouse_move(
            id,
            Point2::new(x, y),
            self.engine.ctx.viewport.width,
            self.engine.ctx.viewport.height,
        );
    }

    pub fn mouse_press(&mut self, id: u64, touch: bool, pressed: bool) {
        let mut input = self.world.components.write_resource::<world::resources::Input>();
        input.mouse_set_pressed(id, touch, pressed);
    }

    pub fn update(&mut self) {
        self.world.update();
        self.engine.deferred_pipeline.update(&self.engine.ctx, &self.world.components);
        self.engine.joystick_pipeline.update(&self.engine.ctx, &self.world.components);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let Some(frame) = self.engine.get_output_frame() {
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            self.engine
                .model_pipeline
                .render(&self.engine.ctx, &self.world.components, &self.engine.deferred_pipeline);

            self.engine
                .deferred_pipeline
                .render(&self.engine.ctx, &self.engine.scaling_pipeline.texture.view);
            self.engine.scaling_pipeline.render(&self.engine.ctx, &view);

            self.engine.glyph_pipeline.render(&self.engine.ctx, &self.world.components, &view);
            self.engine.joystick_pipeline.render(&self.engine.ctx, &view);
        }

        Ok(())
    }
}
