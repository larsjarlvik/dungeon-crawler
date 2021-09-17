use std::time;

use cgmath::*;

use crate::utils::{Interpolate, InterpolatedValue};

pub struct Camera {
    pub target: InterpolatedValue<Vector3<f32>>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub view_proj: Matrix4<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: InterpolatedValue::new(vec3(0.0, 0.0, 0.0)),
            up: Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            view_proj: Matrix4::identity(),
        }
    }
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        let target = InterpolatedValue::new(vec3(0.0, 0.0, 0.0));
        let eye = Point3::new(0.0, 10.0, 6.0);

        let view = Matrix4::look_at_rh(eye, Point3::from_vec(target.current), Vector3::unit_y());
        let proj = perspective(Deg(45.0), aspect, 0.1, 100.0);

        Self {
            target: target.into(),
            up: Vector3::unit_y(),
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            view_proj: proj * view,
        }
    }

    pub fn update(&mut self, last_frame: time::Instant) {
        let target = Point3::from_vec(self.target.get(last_frame));
        let eye = Point3::new(target.x + 0.0, target.y + 10.0, target.z + 6.0);

        self.view_proj = perspective(Deg(45.0), self.aspect, 0.1, 100.0) * Matrix4::look_at_rh(eye, target, Vector3::unit_y());
    }

    pub fn get_eye(&self, last_frame: time::Instant) -> Point3<f32> {
        let target = Point3::from_vec(self.target.get(last_frame));
        Point3::new(target.x + 0.0, target.y + 10.0, target.z + 6.0)
    }
}
