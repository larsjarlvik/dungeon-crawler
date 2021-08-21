// Vertex shader
struct Light {
    position: vec3<f32>;
    direction: vec3<f32>;
    color: vec3<f32>;
};

[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
    light: array<Light, 10>;
    light_count: i32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

var positions: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>( 1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0,-1.0),
    vec2<f32>(-1.0,-1.0),
    vec2<f32>( 1.0,-1.0),
    vec2<f32>( 1.0, 1.0)
);

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coord: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(positions[in_vertex_index], 0.0, 1.0);
    out.tex_coord = positions[in_vertex_index] * 0.5 + 0.5;
    out.tex_coord.y = 1.0 - out.tex_coord.y;
    return out;
}

// Fragment shader
[[group(1), binding(0)]] var t_depth: texture_2d<f32>;
[[group(1), binding(1)]] var t_normal: texture_2d<f32>;
[[group(1), binding(2)]] var t_color: texture_2d<f32>;
[[group(1), binding(3)]] var t_sampler: sampler;

fn world_pos_from_depth(tex_coord: vec2<f32>, depth: f32, inv_matrix: mat4x4<f32>) -> vec3<f32> {
    let ndc = vec3<f32>(vec2<f32>(tex_coord.x, 1.0 - tex_coord.y) * 2.0 - 1.0, depth);
    let p = inv_matrix * vec4<f32>(ndc, 1.0);
    return p.xyz / p.w;
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var depth: f32 = textureSample(t_depth, t_sampler, in.tex_coord).r;
    var color: vec4<f32> = textureSample(t_color, t_sampler, in.tex_coord);
    var normal: vec3<f32> = textureSample(t_normal, t_sampler, in.tex_coord).xyz;

    var light: vec3<f32> = vec3<f32>(0.0);
    for (var i: i32 = 0; i < uniforms.light_count; i = i + 1) {
        light = light + uniforms.light[i].color * max(dot(normal, -uniforms.light[i].direction), 0.0);
    }

    return vec4<f32>(light * color.rgb, color.a);
}
