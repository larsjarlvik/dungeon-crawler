use cgmath::*;

#[derive(Clone, Debug)]
pub enum Interpolation {
    Linear,
    Step,
    CubicSpline,
}

pub trait Interpolate: Copy {
    fn linear(self, other: Self, amount: f32) -> Self;
    fn cubic_spline(source: [Self; 3], source_time: f32, target: [Self; 3], target_time: f32, current_time: f32) -> Self;
}

impl Interpolate for Vector3<f32> {
    fn linear(self, other: Self, amount: f32) -> Self {
        self.lerp(other, amount)
    }

    fn cubic_spline(source: [Self; 3], source_time: f32, target: [Self; 3], target_time: f32, amount: f32) -> Self {
        let t = amount;
        let p0 = source[1];
        let m0 = (target_time - source_time) * source[2];
        let p1 = target[1];
        let m1 = (target_time - source_time) * target[0];

        (2.0 * t * t * t - 3.0 * t * t + 1.0) * p0
            + (t * t * t - 2.0 * t * t + t) * m0
            + (-2.0 * t * t * t + 3.0 * t * t) * p1
            + (t * t * t - t * t) * m1
    }
}

impl Interpolate for Quaternion<f32> {
    fn linear(self, other: Self, amount: f32) -> Self {
        self.slerp(other, amount)
    }

    fn cubic_spline(source: [Self; 3], source_time: f32, target: [Self; 3], target_time: f32, amount: f32) -> Self {
        let t = amount;
        let p0 = source[1];
        let m0 = (target_time - source_time) * source[2];
        let p1 = target[1];
        let m1 = (target_time - source_time) * target[0];

        let result = (2.0 * t * t * t - 3.0 * t * t + 1.0) * p0
            + (t * t * t - 2.0 * t * t + t) * m0
            + (-2.0 * t * t * t + 3.0 * t * t) * p1
            + (t * t * t - t * t) * m1;

        result.normalize()
    }
}
