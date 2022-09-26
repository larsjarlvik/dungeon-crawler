use cgmath::*;

#[derive(Clone, Debug)]
pub struct BoundingSphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl BoundingSphere {
    pub fn transform(&self, transform: Matrix4<f32>) -> Self {
        Self {
            center: transform.transform_point(self.center),
            radius: self.radius,
        }
    }
}

impl Default for BoundingSphere {
    fn default() -> Self {
        Self {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 0.0,
        }
    }
}
