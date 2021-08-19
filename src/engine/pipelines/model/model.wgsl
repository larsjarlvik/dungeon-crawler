// Vertex shader
[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
    model: mat4x4<f32>;
    light_pos: vec3<f32>;
    light_dir: vec3<f32>;
    light_color: vec3<f32>;
    light_ambient: vec3<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] tangent: vec4<f32>;
    [[location(3)]] tex_coord: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] tex_coord: vec2<f32>;
    [[location(2)]] light_pos: vec3<f32>;
    [[location(3)]] light_dir: vec3<f32>;
    [[location(4)]] light_color: vec3<f32>;
    [[location(5)]] light_ambient: vec3<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = model.tex_coord;
    out.normal = model.normal;
    out.clip_position = uniforms.view_proj * uniforms.model * vec4<f32>(model.position, 1.0);
    out.light_pos = uniforms.light_pos;
    out.light_dir = uniforms.light_dir;
    out.light_color = uniforms.light_color;
    out.light_ambient = uniforms.light_ambient;
    return out;
}

// Fragment shader
[[group(1), binding(0)]]
var t_base_color: texture_2d<f32>;
[[group(1), binding(1)]]
var t_normal: texture_2d<f32>;
[[group(1), binding(2)]]
var t_occlusion_roughness_metallic: texture_2d<f32>;
[[group(1), binding(3)]]
var t_sampler: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var base_color: vec4<f32> = textureSample(t_base_color, t_sampler, in.tex_coord);
    let ambient = in.light_color * in.light_ambient;
    let diffuse = in.light_color * max(dot(in.normal, -in.light_dir), 0.0);

    let result = (ambient + diffuse) * base_color.rgb;

    return vec4<f32>(result, base_color.a);
}
