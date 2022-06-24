use bevy_ecs::prelude::*;
use cgmath::*;

use crate::{engine::bounding_sphere, utils::InterpolatedValue};

#[derive(Component, Debug)]
pub struct Light {
    pub color: Vector3<f32>,
    pub base_intensity: f32,
    pub intensity: InterpolatedValue<f32>,
    pub radius: Option<f32>,
    pub bounding_sphere: Option<bounding_sphere::BoundingSphere>,
    pub offset: InterpolatedValue<Vector3<f32>>,
    pub orig_offset: Vector3<f32>,
    pub bloom: f32,
}

impl Light {
    pub fn new(color: Vector3<f32>, intensity: f32, radius: Option<f32>, offset: Vector3<f32>, bloom: f32) -> Self {
        let bounding_sphere = if let Some(radius) = radius {
            Some(bounding_sphere::BoundingSphere {
                center: Point3::from_vec(offset),
                radius: radius * 1.5,
            })
        } else {
            None
        };

        Self {
            color,
            base_intensity: intensity,
            intensity: InterpolatedValue::new(intensity),
            radius,
            bounding_sphere,
            offset: InterpolatedValue::new(offset),
            orig_offset: offset,
            bloom,
        }
    }
}
