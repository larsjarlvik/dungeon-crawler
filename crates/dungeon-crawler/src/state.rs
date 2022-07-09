use crate::{
    views::{self, Views},
    world::{
        self,
        resources::{self, input::PressState},
    },
};
use cgmath::*;
use engine::file;
use std::time::Instant;
use winit::{event::VirtualKeyCode, window::Window};

pub struct State {
    pub engine: engine::Engine,
    pub world: world::World,
    pub views: views::Views,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let start = Instant::now();

        let mut engine = engine::Engine::new(
            &window,
            Point2::new(window.inner_size().width, window.inner_size().height),
            window.scale_factor() as f32,
            file::read_bytes("exo2-medium.ttf"),
        )
        .await;

        let world = world::World::new(&engine);
        let views = Views::new(&mut engine.ctx, window.scale_factor() as f32);

        println!("Startup {} ms", start.elapsed().as_millis());
        Self { engine, world, views }
    }

    pub fn resize(&mut self, window: &Window) {
        if self.engine.ctx.surface.is_some() {
            self.engine.set_viewport(
                window,
                Point2::new(window.inner_size().width, window.inner_size().height),
                window.scale_factor() as f32,
            );

            let size = window.inner_size();
            let pos = window.inner_position().unwrap_or(winit::dpi::PhysicalPosition::new(100, 100));
            let fullscreen = window.fullscreen().is_some();

            if !fullscreen {
                self.engine.ctx.settings.window_size = [size.width, size.height];
                self.engine.ctx.settings.window_pos = [pos.x, pos.y];
            }
            self.engine.ctx.settings.fullscreen = window.fullscreen().is_some();
            self.engine.ctx.settings.store();
            self.engine.shadow_pipeline.resize(&self.engine.ctx);

            self.world.components.remove_resource::<engine::ecs::resources::Camera>().unwrap();
            self.world
                .components
                .insert_resource(engine::ecs::resources::Camera::new(self.engine.ctx.viewport.get_aspect()));
        }
    }

    pub fn keyboard(&mut self, keyboard_input: &winit::event::KeyboardInput) {
        let r = {
            let mut input = self.world.components.get_resource_mut::<world::resources::Input>().unwrap();
            input.keyboard(keyboard_input);
            input.key_state(VirtualKeyCode::R)
        };

        if r == PressState::Pressed(false) {
            self.world.init(&mut self.engine);
        }
    }

    pub fn mouse_move(&mut self, id: u64, x: f32, y: f32) {
        let mut input = self.world.components.get_resource_mut::<world::resources::Input>().unwrap();
        input.mouse_move(
            id,
            Point2::new(x, y),
            self.engine.ctx.viewport.width,
            self.engine.ctx.viewport.height,
        );
    }

    pub fn mouse_press(&mut self, id: u64, touch: bool, pressed: bool) {
        let mut input = self.world.components.get_resource_mut::<resources::Input>().unwrap();
        input.mouse_set_pressed(id, touch, pressed);
    }

    pub fn update(&mut self) {
        self.world.update();
        self.engine.shadow_pipeline.update(&self.engine.ctx, &self.world.components);

        let time = self.world.components.get_resource::<engine::ecs::resources::Time>().unwrap();
        let input = self.world.components.get_resource::<resources::Input>().unwrap();
        let ui_blocking = self.views.update(&mut self.engine.ctx, input, &self.world, time.last_frame);

        if !ui_blocking {
            let (center, current, touch) = if let Some(joystick) = &input.joystick {
                (joystick.center, joystick.current, joystick.touch)
            } else {
                (None, None, false)
            };

            self.engine
                .joystick_pipeline
                .update(&self.engine.ctx, &self.world.components, center, current, touch);
        }

        let mut input = self.world.components.get_resource_mut::<resources::Input>().unwrap();
        input.update();
    }

    pub fn render(&mut self) {
        if let Some((frame, view)) = self.engine.get_output_frame() {
            let anti_aliasing = self
                .engine
                .smaa_target
                .start_frame(&self.engine.ctx.device, &self.engine.ctx.queue, &view);

            self.engine.model_pipeline.render(
                &self.engine.ctx,
                &mut self.world.components,
                &self.engine.shadow_pipeline.texture.view,
                &self.engine.shadow_pipeline.depth_texture.view,
                &self.engine.shadow_pipeline.shadow_texture.view,
            );

            self.engine
                .shadow_pipeline
                .render(&self.engine.ctx, &self.engine.scaling_pipeline.texture.view);

            self.engine.particle_pipeline.render(
                &self.engine.ctx,
                &mut self.world.components,
                &self.engine.scaling_pipeline.texture.view,
                &self.engine.shadow_pipeline.depth_texture.view,
            );

            self.engine.scaling_pipeline.render(&self.engine.ctx, &anti_aliasing);
            anti_aliasing.resolve();

            self.engine.image_pipeline.render(&mut self.engine.ctx, &view);
            self.engine.glyph_pipeline.draw_queued(&mut self.engine.ctx, &view);
            self.engine.joystick_pipeline.render(&self.engine.ctx, &view);

            frame.present();
        }
    }
}
