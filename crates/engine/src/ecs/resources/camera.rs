use crate::{config, frustum};
use bevy_ecs::resource::Resource;
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
    pub rotation: Quaternion<f32>,
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
            rotation: Quaternion::zero(),
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
        let rotation = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));

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
            rotation,
            frustum: frustum::Frustum::from_matrix(view_proj),
        }
    }

    pub fn set(&mut self, target: Vector3<f32>) {
        self.target = target;

        let dist = self
            .rotation
            .rotate_point(point3(0.0, config::CAMERA_DISTANCE, config::CAMERA_DISTANCE * 0.6))
            .to_vec();

        self.eye = Point3::from_vec(target + dist);
        self.proj = perspective(Deg(45.0), self.aspect, 1.0, config::Z_FAR);
        self.view = Matrix4::look_at_rh(self.eye, Point3::from_vec(target), Vector3::unit_y());
        self.view_proj = self.proj * self.view;
        self.frustum = frustum::Frustum::from_matrix(self.view_proj);
    }

    pub fn get_shadow_matrix(&self) -> Matrix4<f32> {
        let light_offset = vec3(-7.0, 40.0, 9.0);
        let shadow_eye = Point3::from_vec(self.target + light_offset);
        let light_view = Matrix4::look_at_rh(shadow_eye, Point3::from_vec(self.target), Vector3::unit_y());

        let inv_vp = self.view_proj.invert().unwrap();
        let mut min_ls = vec3(f32::MAX, f32::MAX, f32::MAX);
        let mut max_ls = vec3(f32::MIN, f32::MIN, f32::MIN);
        for &z in &[-1.0_f32, 1.0] {
            for &y in &[-1.0_f32, 1.0] {
                for &x in &[-1.0_f32, 1.0] {
                    let w = inv_vp * vec4(x, y, z, 1.0);
                    let p = Point3::new(w.x / w.w, (w.y / w.w).max(0.0), w.z / w.w);
                    let ls = light_view.transform_point(p);
                    min_ls = vec3(min_ls.x.min(ls.x), min_ls.y.min(ls.y), min_ls.z.min(ls.z));
                    max_ls = vec3(max_ls.x.max(ls.x), max_ls.y.max(ls.y), max_ls.z.max(ls.z));
                }
            }
        }

        max_ls.z += 15.0;
        let opengl_to_wgpu = Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0);

        let near = (-max_ls.z).max(0.1);
        let far = -min_ls.z;

        opengl_to_wgpu * ortho(min_ls.x, max_ls.x, min_ls.y, max_ls.y, near, far) * light_view
    }
}
