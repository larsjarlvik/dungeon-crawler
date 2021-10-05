use crate::utils::{Interpolate, InterpolatedValue};
use cgmath::*;
use specs::{Component, VecStorage};

#[derive(Debug)]
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

    pub fn to_matrix(&self, frame_time: f32) -> Matrix4<f32> {
        Matrix4::from_translation(self.translation.get(frame_time)) * Matrix4::from(self.rotation.get(frame_time))
    }
}
