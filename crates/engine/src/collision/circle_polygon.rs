use super::Polygon;
use cgmath::*;

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

pub fn circle_polygon(a: &Polygon, center: Vector2<f32>, radius: f32) -> bool {
    (0..a.len()).any(|i| line_circle(a[i], a[(i + 1) % a.len()], center, radius))
}
