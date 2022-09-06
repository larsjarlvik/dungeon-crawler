use cgmath::*;

pub type Polygon = Vec<Vector2<f32>>;

pub trait PolygonMethods {
    fn transform(&self, translation: Vector3<f32>, rot: Quaternion<f32>) -> Polygon;
    fn scale(&self, scale: f32) -> Polygon;
    fn center(&self) -> Vector2<f32>;
    fn radius(&self, center: Vector2<f32>) -> f32;
    fn edges(&self) -> Vec<Vector2<f32>>;
}

impl PolygonMethods for Polygon {
    fn transform(&self, position: Vector3<f32>, rot: Quaternion<f32>) -> Polygon {
        self.iter()
            .map(|p| {
                let pos = vec3(p.x, 0.0, p.y);
                let f = rot.rotate_vector(pos) + position;
                vec2(f.x, f.z)
            })
            .collect()
    }

    fn scale(&self, scale: f32) -> Polygon {
        self.iter().map(|p| vec2(p.x * scale, p.y * scale)).collect()
    }

    fn center(&self) -> Vector2<f32> {
        let mut center = vec2(0.0, 0.0);
        for point in self {
            center += *point;
        }

        center / self.len() as f32
    }

    fn radius(&self, center: Vector2<f32>) -> f32 {
        let mut radius: f32 = 0.0;
        for point in self {
            radius = radius.max(center.distance(*point));
        }

        radius
    }

    fn edges(&self) -> Vec<Vector2<f32>> {
        let mut edges = vec![];

        for i in 0..self.len() {
            let p1 = self[i];
            let p2 = self[(i + 1) % self.len()];
            edges.push(p2 - p1);
        }

        edges
    }
}
