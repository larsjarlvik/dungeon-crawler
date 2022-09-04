use cgmath::*;

#[derive(Debug)]
pub struct InterpolatedValue<T> {
    pub current: T,
    pub previous: Option<T>,
    frame: u32,
}

impl<T> InterpolatedValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            current: value,
            previous: None,
            frame: 0,
        }
    }

    pub fn set(&mut self, value: T, frame: u32)
    where
        T: Copy,
    {
        if frame > self.frame {
            self.previous = Some(self.current);
            self.frame = frame;
        }
        self.current = value;
    }
}

pub trait Interpolate<T> {
    fn get(&self, frame_time: f32) -> T;
}

impl Interpolate<f32> for InterpolatedValue<f32> {
    fn get(&self, factor: f32) -> f32 {
        if let Some(prev_value) = self.previous {
            return self.current * factor + prev_value * (1.0 - factor);
        }

        self.current
    }
}

impl Interpolate<Vector2<f32>> for InterpolatedValue<Vector2<f32>> {
    fn get(&self, factor: f32) -> Vector2<f32> {
        if let Some(prev_value) = self.previous {
            return prev_value.lerp(self.current, factor);
        }

        self.current
    }
}

impl Interpolate<Vector3<f32>> for InterpolatedValue<Vector3<f32>> {
    fn get(&self, factor: f32) -> Vector3<f32> {
        if let Some(prev_value) = self.previous {
            return prev_value.lerp(self.current, factor);
        }

        self.current
    }
}

impl Interpolate<Quaternion<f32>> for InterpolatedValue<Quaternion<f32>> {
    fn get(&self, factor: f32) -> Quaternion<f32> {
        if let Some(prev_value) = self.previous {
            return prev_value.slerp(self.current, factor);
        }

        self.current
    }
}
