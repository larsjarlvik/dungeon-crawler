use cgmath::*;

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
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
            eye: Point3::new(0.0, 0.0, 0.0),
            target: Point3::new(0.0, 0.0, 0.0),
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
        let eye = Point3::new(0.0, 2.0, 3.0);
        let target = Point3::new(0.0, 1.2, 0.0);
        let view = Matrix4::look_at_rh(eye, target, Vector3::unit_y());
        let proj = perspective(Deg(45.0), aspect, 0.1, 100.0);

        Self {
            eye: eye.into(),
            target: target.into(),
            up: Vector3::unit_y(),
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            view_proj: proj * view,
        }
    }
}
