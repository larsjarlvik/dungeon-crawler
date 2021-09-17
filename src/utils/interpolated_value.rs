use crate::config;
use cgmath::*;
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

    pub fn set(&mut self, value: T)
    where
        T: Copy,
    {
        self.previous = Some(self.current);
        self.current = value;
    }
}

pub trait Interpolate<T> {
    fn get(&self, last: time::Instant) -> T;
}

impl Interpolate<Vector3<f32>> for InterpolatedValue<Vector3<f32>> {
    fn get(&self, previous_time: time::Instant) -> Vector3<f32> {
        if let Some(prev_value) = self.previous {
            let factor = previous_time.elapsed().as_secs_f32() / config::time_step().as_secs_f32();
            return prev_value.lerp(self.current, factor);
        }

        self.current
    }
}

impl Interpolate<Quaternion<f32>> for InterpolatedValue<Quaternion<f32>> {
    fn get(&self, previous_time: time::Instant) -> Quaternion<f32> {
        if let Some(prev_value) = self.previous {
            let factor = previous_time.elapsed().as_secs_f32() / config::time_step().as_secs_f32();
            return prev_value.slerp(self.current, factor);
        }

        self.current
    }
}
