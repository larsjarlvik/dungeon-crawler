extern crate cgmath;
use super::{bounding_box, bounding_sphere};
use cgmath::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frustum {
    pub f: [Vector4<f32>; 6],
}

impl Frustum {
    pub fn from_matrix(m: Matrix4<f32>) -> Self {
        Self {
            f: [
                vec4(m.x.w + m.x.x, m.y.w + m.y.x, m.z.w + m.z.x, m.w.w + m.w.x),
                vec4(m.x.w - m.x.x, m.y.w - m.y.x, m.z.w - m.z.x, m.w.w - m.w.x),
                vec4(m.x.w + m.x.y, m.y.w + m.y.y, m.z.w + m.z.y, m.w.w + m.w.y),
                vec4(m.x.w - m.x.y, m.y.w - m.y.y, m.z.w - m.z.y, m.w.w - m.w.y),
                vec4(m.x.w + m.x.z, m.y.w + m.y.z, m.z.w + m.z.z, m.w.w + m.w.z),
                vec4(m.x.w - m.x.z, m.y.w - m.y.z, m.z.w - m.z.z, m.w.w - m.w.z),
            ],
        }
    }

    pub fn test_bounding_box(&self, aab: &bounding_box::BoundingBox) -> bool {
        if self.f[0].x * if self.f[0].x < 0.0 { aab.min.x } else { aab.max.x }
            + self.f[0].y * if self.f[0].y < 0.0 { aab.min.y } else { aab.max.y }
            + self.f[0].z * if self.f[0].z < 0.0 { aab.min.z } else { aab.max.z }
            >= -self.f[0].w
            && self.f[1].x * if self.f[1].x < 0.0 { aab.min.x } else { aab.max.x }
                + self.f[1].y * if self.f[1].y < 0.0 { aab.min.y } else { aab.max.y }
                + self.f[1].z * if self.f[1].z < 0.0 { aab.min.z } else { aab.max.z }
                >= -self.f[1].w
            && self.f[2].x * if self.f[2].x < 0.0 { aab.min.x } else { aab.max.x }
                + self.f[2].y * if self.f[2].y < 0.0 { aab.min.y } else { aab.max.y }
                + self.f[2].z * if self.f[2].z < 0.0 { aab.min.z } else { aab.max.z }
                >= -self.f[2].w
            && self.f[3].x * if self.f[3].x < 0.0 { aab.min.x } else { aab.max.x }
                + self.f[3].y * if self.f[3].y < 0.0 { aab.min.y } else { aab.max.y }
                + self.f[3].z * if self.f[3].z < 0.0 { aab.min.z } else { aab.max.z }
                >= -self.f[3].w
            && self.f[4].x * if self.f[4].x < 0.0 { aab.min.x } else { aab.max.x }
                + self.f[4].y * if self.f[4].y < 0.0 { aab.min.y } else { aab.max.y }
                + self.f[4].z * if self.f[4].z < 0.0 { aab.min.z } else { aab.max.z }
                >= -self.f[4].w
            && self.f[5].x * if self.f[5].x < 0.0 { aab.min.x } else { aab.max.x }
                + self.f[5].y * if self.f[5].y < 0.0 { aab.min.y } else { aab.max.y }
                + self.f[5].z * if self.f[5].z < 0.0 { aab.min.z } else { aab.max.z }
                >= -self.f[5].w
        {
            return true;
        }

        false
    }

    pub fn test_bounding_sphere(&self, bs: &bounding_sphere::BoundingSphere) -> bool {
        for plane in &self.f {
            if plane.truncate().dot(bs.center.to_vec()) + plane.w <= -bs.radius {
                return false;
            }
        }
        true
    }
}

impl Default for Frustum {
    fn default() -> Self {
        Self::from_matrix(Matrix4::identity())
    }
}
