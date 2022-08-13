// Vertex shader
let M_PI = 3.141592653589793;

struct Uniforms {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    model: mat4x4<f32>,
    start_color: vec4<f32>,
    end_color: vec4<f32>,
    time: f32,
    strength: f32,
    size: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) particle_life_speed: vec2<f32>,
    @location(2) particle_pos: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) elapsed: f32,
    @location(1) position: vec2<f32>,
}

@vertex
fn vert_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let life_time = model.particle_life_speed.x;
    let speed = model.particle_life_speed.y;
    let y = (speed * uniforms.time) % life_time + model.particle_pos.y;

    let x = model.particle_pos.x * clamp(life_time - y + 0.5, 0.0, 0.5);
    let z = model.particle_pos.z * clamp(life_time - y + 0.5, 0.0, 0.5);

    var m: mat4x4<f32> = uniforms.model;
    m[3][0] += x + sin(uniforms.time % (life_time * 20.0) * y) * 0.03;
    m[3][1] += y;
    m[3][2] += z + sin(uniforms.time % (life_time * 20.0) * y * 1.2) * 0.03;

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
@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let opacity = 1.0 - pow(distance(in.position, vec2<f32>(0.0, 0.0)) / uniforms.size, 0.5);
    let color = mix(uniforms.start_color, uniforms.end_color, clamp(in.elapsed * 4.0, 0.0, 1.0));
    return vec4<f32>(color.r, color.g, color.b, color.a * opacity * uniforms.strength);
}
