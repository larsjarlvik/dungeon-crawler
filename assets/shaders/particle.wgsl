// Vertex shader
let M_PI = 3.141592653589793;

[[block]]
struct Uniforms {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
    model: mat4x4<f32>;
    start_color: vec4<f32>;
    end_color: vec4<f32>;
    time: f32;
    strength: f32;
    size: f32;
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
    let spread = vec2<f32>(model.particle.z, model.particle.w);

    let y = (speed * uniforms.time) % life_time;
    let x = spread.x * clamp(life_time - y + 0.5, 0.0, 0.5);
    let z = spread.y * clamp(life_time - y + 0.5, 0.0, 0.5);

    var m: mat4x4<f32> = uniforms.model;
    m[3][0] = m[3][0] + x + sin(uniforms.time % (life_time * 20.0) * y) * 0.06;
    m[3][1] = m[3][1] + y;
    m[3][2] = m[3][2] + z + sin(uniforms.time % (life_time * 20.0) * y * 1.2) * 0.06;

    var mv: mat4x4<f32> = uniforms.view * m;
    mv[0][0] = 1.0; mv[0][1] = 0.0; mv[0][2] = 0.0;
    mv[1][0] = 0.0; mv[1][1] = 1.0; mv[1][2] = 0.0;
    mv[2][0] = 0.0; mv[2][1] = 0.0; mv[2][2] = 1.0;

    let position = model.position * uniforms.size;

    out.position = position;
    out.elapsed = y;
    out.clip_position = uniforms.proj * mv * vec4<f32>(position.x, position.y, 0.0, 1.0);

    return out;
}

// Fragment shader
[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let opacity = 1.0 - pow(distance(in.position, vec2<f32>(0.0, 0.0)) / uniforms.size, 0.5);
    let color = mix(uniforms.start_color, uniforms.end_color, clamp(in.elapsed * 2.4, 0.0, 1.0));
    return vec4<f32>(color.r, color.g, color.b, color.a * opacity * uniforms.strength * 0.5);
}
