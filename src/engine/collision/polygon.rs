use cgmath::*;

pub type Polygon = Vec<Vector2<f32>>;

pub trait PolygonMethods {
    fn transform(&self, translation: Option<Vector3<f32>>) -> Polygon;
    fn center(&self) -> Vector2<f32>;
    fn edges(&self) -> Vec<Vector2<f32>>;
}

impl PolygonMethods for Polygon {
    fn transform(&self, position: Option<Vector3<f32>>) -> Polygon {
        if let Some(position) = position {
            return self.iter().map(|p| Vector2::new(position.x + p.x, position.z + p.y)).collect();
        }

        self.to_vec()
    }

    fn center(&self) -> Vector2<f32> {
        let mut center = vec2(0.0, 0.0);
        for point in self {
            center += *point;
        }

        center / self.len() as f32
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
