pub use self::polygon::*;
use cgmath::*;
mod polygon;

#[derive(Debug)]
pub enum Intersection {
    None,
    WillIntersect(Vector2<f32>),
    Intersect,
}

pub fn check_collision(a: &Polygon, b: &Polygon, velocity: Vector2<f32>) -> Intersection {
    let mut intersect = true;
    let mut will_intersect = true;

    let a_e = a.edges();
    let b_e = b.edges();

    let mut min_interval_distance = f32::MAX;
    let mut translation_axis = vec2(0.0, 0.0);

    for i in 0..(a_e.len() + b_e.len()) {
        let edge = if i < a_e.len() { a_e[i] } else { b_e[i - a_e.len()] };
        let axis = vec2(-edge.y, edge.x).normalize();

        let (mut min_a, mut max_a) = project_polygon(axis, a);
        let (min_b, max_b) = project_polygon(axis, b);

        if interval_distance(min_a, max_a, min_b, max_b) > 0.0 {
            intersect = false;
        }

        let velocity_projection = axis.dot(velocity);
        if velocity_projection < 0.0 {
            min_a += velocity_projection;
        } else {
            max_a += velocity_projection;
        }

        let mut interval_distance = interval_distance(min_a, max_a, min_b, max_b);
        if interval_distance > 0.0 {
            will_intersect = false;
        }

        if !intersect && !will_intersect {
            break;
        }

        interval_distance = interval_distance.abs();
        if interval_distance < min_interval_distance {
            min_interval_distance = interval_distance;
            translation_axis = axis;

            let d = a.center() - b.center();
            if d.dot(translation_axis) < 0.0 {
                translation_axis = -translation_axis;
            }
        }
    }

    if will_intersect {
        return Intersection::WillIntersect(translation_axis * min_interval_distance);
    }

    if intersect {
        return Intersection::Intersect;
    }

    Intersection::None
}

fn project_polygon(axis: Vector2<f32>, polygon: &Polygon) -> (f32, f32) {
    let mut dot_product = axis.dot(polygon[0]);
    let mut min = dot_product;
    let mut max = dot_product;

    for point in polygon {
        dot_product = point.dot(axis);
        if dot_product < min {
            min = dot_product;
        } else if dot_product > max {
            max = dot_product;
        }
    }

    (min, max)
}

fn interval_distance(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> f32 {
    if min_a < min_b {
        min_b - max_a
    } else {
        min_a - max_b
    }
}
