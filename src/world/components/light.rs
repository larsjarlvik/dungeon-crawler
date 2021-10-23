use cgmath::*;
use specs::{Component, VecStorage};

use crate::{engine::bounding_box, utils::InterpolatedValue};

#[derive(Debug)]
pub struct Light {
    pub color: Vector3<f32>,
    pub base_intensity: f32,
    pub intensity: InterpolatedValue<f32>,
    pub radius: Option<f32>,
    pub bounding_box: Option<bounding_box::BoundingBox>,
    pub offset: Vector3<f32>,
}

impl Light {
    pub fn new(color: Vector3<f32>, intensity: f32, radius: Option<f32>, offset: Vector3<f32>) -> Self {
        let bounding_box = if let Some(radius) = radius {
            let radius = radius / 4.0;
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
            offset,
        }
    }
}

impl Component for Light {
    type Storage = VecStorage<Self>;
}
