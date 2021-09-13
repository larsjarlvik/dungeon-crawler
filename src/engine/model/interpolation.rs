use cgmath::*;

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
        let left = self;
        let right = other;

        let num2;
        let num3;
        let num = amount;
        let mut num4 = (((left.v.x * right.v.x) + (left.v.y * right.v.y)) + (left.v.z * right.v.z)) + (left.s * right.s);
        let mut flag = false;
        if num4 < 0.0 {
            flag = true;
            num4 = -num4;
        }
        if num4 > 0.999_999 {
            num3 = 1.0 - num;
            num2 = if flag { -num } else { num };
        } else {
            let num5 = num4.acos();
            let num6 = 1.0 / num5.sin();
            num3 = ((1.0 - num) * num5).sin() * num6;
            num2 = if flag {
                -(num * num5).sin() * num6
            } else {
                (num * num5).sin() * num6
            };
        }
        Quaternion::new(
            (num3 * left.s) + (num2 * right.s),
            (num3 * left.v.x) + (num2 * right.v.x),
            (num3 * left.v.y) + (num2 * right.v.y),
            (num3 * left.v.z) + (num2 * right.v.z),
        )
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
