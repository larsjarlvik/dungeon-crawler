use crate::{engine::pipelines::ParticleEmitter, utils::InterpolatedValue};
use cgmath::*;
use specs::{Component, VecStorage};

pub struct Particle {
    pub start_color: Vector3<f32>,
    pub end_color: Vector3<f32>,
    pub emitter: ParticleEmitter,
    pub size: f32,
    pub base_strength: f32,
    pub strength: InterpolatedValue<f32>,
}

impl Particle {
    pub fn new(emitter: ParticleEmitter, start_color: Vector3<f32>, end_color: Vector3<f32>, size: f32, strength: f32) -> Self {
        Self {
            emitter,
            start_color,
            end_color,
            size,
            base_strength: strength,
            strength: InterpolatedValue::new(strength),
        }
    }
}

impl Component for Particle {
    type Storage = VecStorage<Self>;
}
