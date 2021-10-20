use crate::engine::pipelines::ParticleEmitter;
use cgmath::*;
use specs::{Component, VecStorage};

pub struct Particle {
    pub start_color: Vector3<f32>,
    pub end_color: Vector3<f32>,
    pub emitter: ParticleEmitter,
}

impl Particle {
    pub fn new(emitter: ParticleEmitter, start_color: Vector3<f32>, end_color: Vector3<f32>) -> Self {
        Self {
            emitter,
            start_color,
            end_color,
        }
    }
}

impl Component for Particle {
    type Storage = VecStorage<Self>;
}
