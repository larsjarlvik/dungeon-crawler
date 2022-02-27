use crate::{
    engine::{bounding_box, pipelines::ParticleEmitter},
    utils::InterpolatedValue,
};
use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Component)]
pub struct Particle {
    pub start_color: Vector3<f32>,
    pub end_color: Vector3<f32>,
    pub emitter: ParticleEmitter,
    pub size: f32,
    pub base_strength: f32,
    pub strength: InterpolatedValue<f32>,
    pub bounding_box: bounding_box::BoundingBox,
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
            bounding_box: bounding_box::BoundingBox {
                min: point3(-size, -size, -size),
                max: point3(size, size, size),
            },
        }
    }
}
