use crate::{config, frustum};
use bevy_ecs::system::Resource;
use cgmath::*;

#[derive(Resource)]
pub struct Camera {
    pub target: Vector3<f32>,
    pub eye: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>,
    pub view_proj: Matrix4<f32>,
    pub frustum: frustum::Frustum,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: vec3(0.0, 0.0, 0.0),
            eye: point3(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: config::Z_FAR,
            view: Matrix4::identity(),
            proj: Matrix4::identity(),
            view_proj: Matrix4::identity(),
            frustum: frustum::Frustum::default(),
        }
    }
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        let target = vec3(0.0, 0.0, 0.0);
        let eye = point3(0.0, 10.0, 6.0);

        let view = Matrix4::look_at_rh(eye, Point3::from_vec(target), Vector3::unit_y());
        let proj = perspective(Deg(45.0), aspect, 1.0, config::Z_FAR);
        let view_proj = proj * view;

        Self {
            target,
            eye,
            up: Vector3::unit_y(),
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: config::Z_FAR,
            view,
            proj,
            view_proj,
            frustum: frustum::Frustum::from_matrix(view_proj),
        }
    }

    pub fn set(&mut self, target: Vector3<f32>) {
        self.target = target;

        let rot = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));
        let dist = rot
            .rotate_point(point3(0.0, config::CAMERA_DISTANCE, config::CAMERA_DISTANCE * 0.6))
            .to_vec();

        self.eye = Point3::from_vec(target + dist);
        self.proj = perspective(Deg(45.0), self.aspect, 1.0, config::Z_FAR);
        self.view = Matrix4::look_at_rh(self.eye, Point3::from_vec(target), Vector3::unit_y());
        self.view_proj = self.proj * self.view;
        self.frustum = frustum::Frustum::from_matrix(self.view_proj);
    }

    pub fn get_shadow_matrix(&self) -> Matrix4<f32> {
        let shadow_eye = point3(self.target.x - 7.0, self.target.y + 40.0, self.target.z + 9.0);
        perspective(Deg(45.0), self.aspect, 5.0, 100.0) * Matrix4::look_at_rh(shadow_eye, Point3::from_vec(self.target), Vector3::unit_y())
    }
}
