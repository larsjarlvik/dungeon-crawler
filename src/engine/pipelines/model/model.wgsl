// Vertex shader
[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
    model: mat4x4<f32>;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] tangent: vec4<f32>;
    [[location(3)]] tex_coord: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] position: vec4<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let pos = uniforms.model * vec4<f32>(model.position, 1.0);

    out.position = pos;
    out.tex_coord = model.tex_coord;
    out.normal = model.normal;
    out.clip_position = uniforms.view_proj * pos;
    return out;
}

// Fragment shader
struct GBufferOutput {
  [[location(0)]] position : vec4<f32>;
  [[location(1)]] normal : vec4<f32>;
  [[location(2)]] color : vec4<f32>;
};

[[group(1), binding(0)]] var t_base_color: texture_2d<f32>;
[[group(1), binding(1)]] var t_normal: texture_2d<f32>;
[[group(1), binding(2)]] var t_occlusion_roughness_metallic: texture_2d<f32>;
[[group(1), binding(3)]] var t_sampler: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> GBufferOutput {
    var output : GBufferOutput;

    var base_color: vec4<f32> = textureSample(t_base_color, t_sampler, in.tex_coord);
    output.position = in.position;
    output.normal = vec4<f32>(in.normal, 1.0);
    output.color = base_color;
    return output;
}
