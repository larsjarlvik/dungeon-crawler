use cgmath::*;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl BoundingBox {
    pub fn transform(&self, transform: Matrix4<f32>) -> Self {
        let b1: Point3<f32> = transform.transform_point(self.min);
        let b2: Point3<f32> = transform.transform_point(self.max);

        Self {
            min: Point3::new(b1.x.min(b2.x), b1.y.min(b2.y), b1.z.min(b2.z)),
            max: Point3::new(b1.x.max(b2.x), b1.y.max(b2.y), b1.z.max(b2.z)),
        }
    }

    pub fn grow(&self, bounding_box: &Self) -> Self {
        Self {
            min: Point3::new(
                self.min.x.min(bounding_box.min.x),
                self.min.y.min(bounding_box.min.y),
                self.min.z.min(bounding_box.min.z),
            ),
            max: Point3::new(
                self.max.x.max(bounding_box.max.x),
                self.max.y.max(bounding_box.max.y),
                self.max.z.max(bounding_box.max.z),
            ),
        }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: Point3::new(0.0, 0.0, 0.0),
            max: Point3::new(0.0, 0.0, 0.0),
        }
    }
}
