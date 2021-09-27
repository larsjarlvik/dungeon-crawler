use crate::config;
use cgmath::*;

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

    pub fn set(&mut self, value: T)
    where
        T: Copy,
    {
        self.previous = Some(self.current);
        self.current = value;
    }

    pub fn freeze(&mut self) {
        self.previous = None;
    }
}

pub trait Interpolate<T> {
    fn get(&self, frame_time: f32) -> T;
}

impl Interpolate<Vector3<f32>> for InterpolatedValue<Vector3<f32>> {
    fn get(&self, frame_time: f32) -> Vector3<f32> {
        if let Some(prev_value) = self.previous {
            let factor = frame_time / config::time_step().as_secs_f32();
            return prev_value.lerp(self.current, factor);
        }

        self.current
    }
}

impl Interpolate<Quaternion<f32>> for InterpolatedValue<Quaternion<f32>> {
    fn get(&self, frame_time: f32) -> Quaternion<f32> {
        if let Some(prev_value) = self.previous {
            let factor = frame_time / config::time_step().as_secs_f32();
            return prev_value.slerp(self.current, factor);
        }

        self.current
    }
}
