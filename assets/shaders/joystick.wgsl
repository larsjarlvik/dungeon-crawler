// Vertex shader
[[block]]
struct Uniforms {
    center: vec2<f32>;
    current: vec2<f32>;
    radius: f32;
    aspect: f32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] coord: vec2<f32>;
};

[[stage(vertex)]]
fn main([[builtin(vertex_index)]] vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let x = i32(vertex_index) / 2;
    let y = i32(vertex_index) & 1;
    let tc = vec2<f32>(f32(x) * 2.0, f32(y) * 2.0);

    let radius = uniforms.radius * 2.0;

    out.clip_position = vec4<f32>(tc.x - 1.0, 1.0 - tc.y, 0.0, 1.0)
        * vec4<f32>(radius, radius * uniforms.aspect, 1.0, 1.0)
        + vec4<f32>(uniforms.center.x, -uniforms.center.y, 0.0, 0.0);
    out.coord = vec2<f32>((tc.x * 2.0) - 2.0, (tc.y * 2.0) - 2.0);

    return out;
}

// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let fade = (1.0 - length(in.coord.xy)) + 0.3;
    let outer = step(length(in.coord.xy), 1.0);
    let inner = step(length(in.coord.xy), 0.95) * fade;
    let center = clamp(outer - inner, 0.0, 1.0) * 0.45;

    let current = step(length(uniforms.current * 0.95 - in.coord.xy), 0.4) * 0.25;
    return vec4<f32>(0.0, 0.0, 0.0, (center + current) * 2.0);
}
