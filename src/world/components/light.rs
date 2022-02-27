use bevy_ecs::prelude::*;
use cgmath::*;

use crate::{engine::bounding_box, utils::InterpolatedValue};

#[derive(Component, Debug)]
pub struct Light {
    pub color: Vector3<f32>,
    pub base_intensity: f32,
    pub intensity: InterpolatedValue<f32>,
    pub radius: Option<f32>,
    pub bounding_box: Option<bounding_box::BoundingBox>,
    pub offset: InterpolatedValue<Vector3<f32>>,
    pub orig_offset: Vector3<f32>,
    pub bloom: f32,
}

impl Light {
    pub fn new(color: Vector3<f32>, intensity: f32, radius: Option<f32>, offset: Vector3<f32>, bloom: f32) -> Self {
        let bounding_box = if let Some(radius) = radius {
            Some(bounding_box::BoundingBox {
                min: point3(-radius, -radius, -radius),
                max: point3(radius, radius, radius),
            })
        } else {
            None
        };

        Self {
            color,
            base_intensity: intensity,
            intensity: InterpolatedValue::new(intensity),
            radius,
            bounding_box,
            offset: InterpolatedValue::new(offset),
            orig_offset: offset,
            bloom,
        }
    }
}
