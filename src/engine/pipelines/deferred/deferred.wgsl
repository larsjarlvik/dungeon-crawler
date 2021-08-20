// Vertex shader
[[block]]
struct Uniforms {
    light_pos: vec3<f32>;
    light_dir: vec3<f32>;
    light_color: vec3<f32>;
    light_ambient: vec3<f32>;
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
[[group(1), binding(1)]] var t_position: texture_2d<f32>;
[[group(1), binding(2)]] var t_normal: texture_2d<f32>;
[[group(1), binding(3)]] var t_color: texture_2d<f32>;
[[group(1), binding(4)]] var t_sampler: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color: vec4<f32> = textureSample(t_color, t_sampler, in.tex_coord);
    var normal: vec3<f32> = textureSample(t_normal, t_sampler, in.tex_coord).xyz;

    let ambient = uniforms.light_color * uniforms.light_ambient;
    let diffuse = uniforms.light_color * max(dot(normal, -uniforms.light_dir), 0.0);

    return vec4<f32>((ambient + diffuse) * color.rgb, color.a);
}
