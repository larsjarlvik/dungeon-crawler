// Vertex shader
[[block]]
struct Uniforms {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
    model: mat4x4<f32>;
    start_color: vec4<f32>;
    end_color: vec4<f32>;
    speed: f32;
    time: f32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] particle: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] elapsed: f32;
    [[location(1)]] position: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;


    let speed = model.particle.y;
    let life_time = model.particle.x;
    let spread_x = model.particle.z;
    let spread_z = model.particle.w;

    let y = (speed * uniforms.time) % life_time;
    let x = spread_x * (y + 0.3) * speed;
    let z = spread_z * (y + 0.3) * speed;

    var m: mat4x4<f32> = uniforms.model;
    m[3][0] = m[3][0] + x;
    m[3][1] = m[3][1] + y;
    m[3][2] = m[3][2] + z;

    var mv: mat4x4<f32> = uniforms.view * m;
    mv[0][0] = 1.0; mv[0][1] = 0.0; mv[0][2] = 0.0;
    mv[1][0] = 0.0; mv[1][1] = 1.0; mv[1][2] = 0.0;
    mv[2][0] = 0.0; mv[2][1] = 0.0; mv[2][2] = 1.0;

    out.position = model.position;
    out.elapsed = uniforms.time % life_time;
    out.clip_position = uniforms.proj * mv * vec4<f32>(model.position.x, model.position.y, 0.0, 1.0);

    return out;
}

// Fragment shader
[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let opacity = 1.0 - (distance(in.position, vec2<f32>(0.0, 0.0)) / 0.02);
    let color = mix(uniforms.start_color, uniforms.end_color, in.elapsed);
    return vec4<f32>(color.r, color.g, color.b, color.a * opacity);
}
