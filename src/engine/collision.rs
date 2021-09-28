use cgmath::*;

pub fn has_collided(poly1: &[Vector2<f32>], poly2: &[Vector2<f32>], max_dist: &Option<f32>) -> bool {
    let estimated_dist = (poly1[1].x - poly2[0].x).powi(2) + (poly1[1].y - poly2[0].y).powi(2);
    match max_dist {
        &Some(max_dist) if estimated_dist > max_dist.powi(2) => false,
        &Some(_) | &None => run_sat(poly1, poly2),
    }
}

fn run_sat(poly1: &[Vector2<f32>], poly2: &[Vector2<f32>]) -> bool {
    let mut edges = Vec::new();
    edges.append(&mut poly_to_edges(&poly1));
    edges.append(&mut poly_to_edges(&poly2));

    let axes = edges.into_iter().map(orthogonal);

    for axis in axes {
        if !overlap(project(&poly1, axis), project(&poly2, axis)) {
            return false;
        }
    }

    true
}

fn edge_vector(point1: Vector2<f32>, point2: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(point2.x - point1.x, point2.y - point1.y)
}

fn poly_to_edges(poly: &[Vector2<f32>]) -> Vec<Vector2<f32>> {
    let mut edges = Vec::with_capacity(poly.len());

    for index in 0..poly.len() {
        edges.push(edge_vector(poly[index], poly[(index + 1) % poly.len()]));
    }

    edges
}

fn orthogonal(vector: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(vector.y, -vector.x)
}

fn dot_product(vector1: Vector2<f32>, vector2: Vector2<f32>) -> f32 {
    vector1.x * vector2.x + vector1.y * vector2.y
}

fn project(poly: &[Vector2<f32>], axis: Vector2<f32>) -> Vector2<f32> {
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;

    for point in poly.iter() {
        let dot = dot_product(*point, axis);

        match min {
            Some(val) if val < dot => (),
            _ => min = Some(dot),
        }
        match max {
            Some(val) if val > dot => (),
            _ => max = Some(dot),
        }
    }

    return Vector2::new(min.unwrap(), max.unwrap());
}

fn overlap(projection1: Vector2<f32>, projection2: Vector2<f32>) -> bool {
    return projection1.x <= projection2.y && projection2.x <= projection1.y;
}
