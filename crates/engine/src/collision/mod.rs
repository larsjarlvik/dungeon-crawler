pub use self::polygon::*;
use cgmath::*;
mod polygon;

#[derive(Debug, PartialEq, Clone, Copy)]
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

fn line_point(a: Vector2<f32>, b: Vector2<f32>, p: Vector2<f32>) -> bool {
    let d1 = p.distance(a);
    let d2 = p.distance(b);
    let line_len = a.distance(b);
    let buffer = 0.001;

    d1 + d2 >= line_len - buffer && d1 + d2 <= line_len + buffer
}

fn point_circle(p: Vector2<f32>, center: Vector2<f32>, radius: f32) -> bool {
    let dist_x = p.x - center.x;
    let dist_y = p.y - center.y;

    ((dist_x * dist_x) + (dist_y * dist_y)).sqrt() <= radius
}

fn line_circle(a: Vector2<f32>, b: Vector2<f32>, center: Vector2<f32>, radius: f32) -> bool {
    if point_circle(a, center, radius) || point_circle(b, center, radius) {
        return true;
    };

    let len = ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt();
    let dot = (((center.x - a.x) * (b.x - a.x)) + ((center.y - a.y) * (b.y - a.y))) / len.powf(2.0);
    let closest = vec2(a.x + (dot * (b.x - a.x)), a.y + (dot * (b.y - a.y)));
    if !line_point(a, b, closest) {
        return false;
    };

    let distance = ((closest.x - center.x).powf(2.0) + (closest.y - center.y).powf(2.0)).sqrt();
    distance <= radius
}

pub fn check_collision_circle(a: &Polygon, center: Vector2<f32>, radius: f32) -> bool {
    (0..a.len()).any(|i| line_circle(a[i], a[(i + 1) % a.len()], center, radius))
}

pub fn check_collision_array(position: Vector3<f32>, collider: &Polygon, collisions: &[Polygon]) -> bool {
    for collision in collisions.iter() {
        let result = check_collision(collider, collision, vec2(position.x, position.z));

        match result {
            Intersection::None => {}
            Intersection::Intersect | Intersection::WillIntersect(_) => return true,
        }
    }

    false
}

pub fn get_collision_offset(position: Vector3<f32>, collider: &Polygon, collisions: &[Polygon]) -> Vector3<f32> {
    let mut offset = position;
    let mut hits = 0;

    for collision in collisions.iter() {
        let collision_center = collision.center();
        let collider_center = collider.center();
        if collision_center.distance(collider_center) > collision.radius(collision_center) + collider.radius(collider_center) {
            continue;
        }

        let result = check_collision(collider, collision, vec2(position.x, position.z));
        offset = match result {
            Intersection::WillIntersect(mtv) => {
                hits += 1;
                offset + position + vec3(mtv.x, 0.0, mtv.y)
            }
            _ => offset,
        };
    }

    offset / (hits + 1) as f32
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
    let d1 = min_b - max_a;
    let d2 = min_a - max_b;
    let sign = if min_a < min_b { d1 / d1.abs() } else { d2 / d2.abs() };
    sign * d1.abs().min(d2.abs())
}
