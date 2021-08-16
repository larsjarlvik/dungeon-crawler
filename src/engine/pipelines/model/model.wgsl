// Vertex shader
[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
    model: mat4x4<f32>;
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
    [[location(0)]] tex_coord: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = model.tex_coord;
    out.clip_position = uniforms.view_proj * uniforms.model * vec4<f32>(model.position, 1.0);
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
    return base_color;
}
