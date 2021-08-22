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
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] normal_w: vec3<f32>;
    [[location(2)]] tangent_w: vec3<f32>;
    [[location(3)]] bitangent_w: vec3<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let pos = uniforms.model * vec4<f32>(model.position, 1.0);

    var t: vec4<f32> = normalize(model.tangent);
    out.normal_w = normalize((uniforms.model * vec4<f32>(model.normal, 0.0)).xyz);
    out.tangent_w = normalize((uniforms.model * model.tangent).xyz);
    out.bitangent_w = cross(out.normal_w, out.tangent_w) * t.w;

    out.clip_position = uniforms.view_proj * pos;
    out.tex_coord = model.tex_coord;
    return out;
}

// Fragment shader
struct GBufferOutput {
  [[location(0)]] normal : vec4<f32>;
  [[location(1)]] color : vec4<f32>;
  [[location(2)]] orm : vec4<f32>;
};

[[group(1), binding(0)]] var t_base_color: texture_2d<f32>;
[[group(1), binding(1)]] var t_normal: texture_2d<f32>;
[[group(1), binding(2)]] var t_occlusion_roughness_metallic: texture_2d<f32>;
[[group(1), binding(3)]] var t_sampler: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> GBufferOutput {
    var output : GBufferOutput;

    output.color = textureSample(t_base_color, t_sampler, in.tex_coord);
    output.orm = textureSample(t_occlusion_roughness_metallic, t_sampler, in.tex_coord);

    var tangent: mat3x3<f32> = mat3x3<f32>(in.tangent_w, in.bitangent_w, in.normal_w);
    var normal: vec3<f32> = textureSample(t_normal, t_sampler, in.tex_coord).xyz;
    output.normal = vec4<f32>(normalize(tangent * (2.0 * normal - 1.0)), 1.0);
    return output;
}
