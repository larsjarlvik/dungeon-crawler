use cgmath::*;

#[derive(Clone)]
pub struct BoundingBox {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl BoundingBox {
    pub fn new() -> Self {
        Self {
            min: Point3::new(0.0, 0.0, 0.0),
            max: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn transform(&self, transform: [[f32; 4]; 4]) -> Self {
        let b1: Point3<f32> = cgmath::Matrix4::from(transform).transform_point(self.min);
        let b2: Point3<f32> = cgmath::Matrix4::from(transform).transform_point(self.max);

        Self {
            min: Point3::new(
                if b1.x < b2.x { b1.x } else { b2.x },
                if b1.y < b2.y { b1.y } else { b2.y },
                if b1.z < b2.z { b1.z } else { b2.z },
            ),
            max: Point3::new(
                if b1.x > b2.x { b1.x } else { b2.x },
                if b1.y > b2.y { b1.y } else { b2.y },
                if b1.z > b2.z { b1.z } else { b2.z },
            ),
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
