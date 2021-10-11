use cgmath::*;

fn mix(n1: f32, n2: f32, amount: f32) -> f32 {
    vec1(n1).lerp(vec1(n2), amount).x
}

fn hash(n: f32) -> f32 {
    (n.sin() * 1e4).fract()
}

fn noise(x: f32) -> f32 {
    let i = x.floor();
    let f = x.fract();
    let u = f * f * (3.0 - 2.0 * f);

    mix(hash(i), hash(i + 1.0), u)
}

pub fn fbm(x: f32, octaves: usize) -> f32 {
    let mut x = x;
    let mut v = 0.0;
    let mut a = 0.5;
    let shift = 100.0;
    for _ in 0..octaves {
        v += a * noise(x);
        x = x * 2.0 + shift;
        a *= 0.5;
    }
    v
}
