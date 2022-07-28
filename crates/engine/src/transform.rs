use crate::interpolated_value::{Interpolate, InterpolatedValue};
use cgmath::*;

pub struct Transform {
    pub translation: InterpolatedValue<Vector3<f32>>,
    pub rotation: InterpolatedValue<Quaternion<f32>>,
    pub scale: InterpolatedValue<Vector3<f32>>,
}

impl Transform {
    #[allow(dead_code)]
    pub fn from_translation(translation: Vector3<f32>) -> Self {
        Self {
            translation: InterpolatedValue::new(translation),
            rotation: InterpolatedValue::new(Quaternion::from_angle_y(Rad(0.0))),
            scale: InterpolatedValue::new(vec3(1.0, 1.0, 1.0)),
        }
    }

    pub fn from_translation_scale(translation: Vector3<f32>, scale: f32) -> Self {
        Self {
            translation: InterpolatedValue::new(translation),
            rotation: InterpolatedValue::new(Quaternion::from_angle_y(Rad(0.0))),
            scale: InterpolatedValue::new(vec3(scale, scale, scale)),
        }
    }

    pub fn from_translation_angle(translation: Vector3<f32>, angle: f32) -> Self {
        Self {
            translation: InterpolatedValue::new(translation),
            rotation: InterpolatedValue::new(Quaternion::from_angle_y(Deg(angle))),
            scale: InterpolatedValue::new(vec3(1.0, 1.0, 1.0)),
        }
    }

    pub fn to_matrix(&self, frame_time: f32) -> Matrix4<f32> {
        let scale = self.scale.get(frame_time);
        Matrix4::from_translation(self.translation.get(frame_time))
            * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z)
            * Matrix4::from(self.rotation.get(frame_time))
    }
}
