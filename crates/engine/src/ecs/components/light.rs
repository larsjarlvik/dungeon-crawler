use crate::{bounding_sphere, interpolated_value::InterpolatedValue};
use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Component, Debug)]
pub struct Light {
    pub color: Vector3<f32>,
    pub base_intensity: f32,
    pub intensity: InterpolatedValue<f32>,
    pub radius: f32,
    pub bounding_sphere: bounding_sphere::BoundingSphere,
    pub offset: InterpolatedValue<Vector3<f32>>,
    pub orig_offset: Vector3<f32>,
    pub bloom: f32,
}

impl Light {
    pub fn new(color: Vector3<f32>, intensity: f32, radius: f32, offset: Vector3<f32>, bloom: f32) -> Self {
        let bounding_sphere = bounding_sphere::BoundingSphere {
            center: Point3::from_vec(offset),
            radius: radius * 0.75,
        };

        Self {
            color,
            base_intensity: intensity * 4.0,
            intensity: InterpolatedValue::new(intensity * 4.0),
            radius,
            bounding_sphere,
            offset: InterpolatedValue::new(offset),
            orig_offset: offset,
            bloom,
        }
    }
}
