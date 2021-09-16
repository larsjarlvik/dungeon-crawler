use crate::config;
use cgmath::*;
use specs::{Component, VecStorage};
use std::time;

pub struct InterpolatedValue<T> {
    pub current: T,
    pub previous: Option<T>,
}

impl<T> InterpolatedValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            current: value,
            previous: None,
        }
    }
}

pub struct Transform {
    pub translation: InterpolatedValue<Vector3<f32>>,
    pub rotation: InterpolatedValue<Quaternion<f32>>,
    pub scale: InterpolatedValue<Vector3<f32>>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn from_translation(translation: Vector3<f32>) -> Self {
        Self {
            translation: InterpolatedValue::new(translation),
            rotation: InterpolatedValue::new(Quaternion::from_angle_y(Rad(0.0))),
            scale: InterpolatedValue::new(vec3(1.0, 1.0, 1.0)),
        }
    }

    pub fn set_translation(&mut self, value: Vector3<f32>) {
        self.translation.previous = Some(self.translation.current);
        self.translation.current = value;
    }

    pub fn set_rotation(&mut self, value: f32) {
        self.rotation.previous = Some(self.rotation.current);
        self.rotation.current = Quaternion::from_angle_y(Rad(value));
    }

    pub fn get_translation(&self, previous_time: time::Instant) -> Vector3<f32> {
        if let Some(prev_value) = self.translation.previous {
            let factor = previous_time.elapsed().as_secs_f32() / config::time_step().as_secs_f32();
            return prev_value.lerp(self.translation.current, factor);
        }
        self.translation.current
    }

    pub fn get_rotation(&self, previous_time: time::Instant) -> Quaternion<f32> {
        if let Some(prev_value) = self.rotation.previous {
            let factor = previous_time.elapsed().as_secs_f32() / config::time_step().as_secs_f32();
            return prev_value.slerp(self.rotation.current, factor);
        }
        self.rotation.current
    }
}
