use cgmath::*;

use crate::{config, engine::frustum};

pub struct Camera {
    pub target: Vector3<f32>,
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
            up: Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: config::Z_FAR,
            view: Matrix4::identity(),
            proj: Matrix4::identity(),
            view_proj: Matrix4::identity(),
            frustum: frustum::Frustum::new(),
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
        let dist = rot.rotate_point(point3(0.0, 10.0, 6.0)).to_vec();
        let eye = Point3::from_vec(target + dist);

        self.proj = perspective(Deg(45.0), self.aspect, 1.0, config::Z_FAR);
        self.view = Matrix4::look_at_rh(eye, Point3::from_vec(target), Vector3::unit_y());
        self.view_proj = self.proj * self.view;
        self.frustum = frustum::Frustum::from_matrix(self.view_proj);
    }

    pub fn get_eye(&self) -> Point3<f32> {
        Point3::new(self.target.x + 0.0, self.target.y + 10.0, self.target.z + 6.0)
    }

    pub fn get_shadow_matrix(&self) -> Matrix4<f32> {
        let inv_cam = (self.view_proj).inverse_transform().unwrap();

        #[rustfmt::skip]
        let mut frustum_corners = [
            vec3(-1.0, 1.0,-1.0),
            vec3( 1.0, 1.0,-1.0),
            vec3( 1.0,-1.0,-1.0),
            vec3(-1.0,-1.0,-1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3( 1.0, 1.0, 1.0),
            vec3( 1.0,-1.0, 1.0),
            vec3(-1.0,-1.0, 1.0),
        ];

        let mut center = Vector3::zero();
        for i in 0..8 {
            let inverted = inv_cam * frustum_corners[i].extend(1.0);
            frustum_corners[i] = inverted.truncate() / inverted.w;
            center += frustum_corners[i];
        }
        center /= 8.0;

        let mut radius = 0.0f32;
        for corner in frustum_corners.iter() {
            radius = radius.max(corner.distance(center));
        }

        let light_dir = vec3(0.1, -1.0, 0.1);
        let light_view_matrix = Matrix4::look_at_rh(
            Point3::from_vec(center - light_dir * radius),
            Point3::from_vec(center),
            Vector3::unit_y(),
        );

        let mut light_ortho_matrix = Matrix4::zero();
        light_ortho_matrix[0][0] = 2.0 / radius;
        light_ortho_matrix[1][1] = 2.0 / radius;
        light_ortho_matrix[2][2] = -1.0 / radius;
        light_ortho_matrix[3][3] = 1.0;

        light_ortho_matrix * light_view_matrix
    }
}
