struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec4<f32>,
    @location(3) tex_coord: vec2<f32>,
    @location(4) weights: vec4<f32>,
    @location(5) joints: vec4<u32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
}

@vertex
fn vert_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.normal = normalize(vec4(model.normal, 0.0) * uniforms.model).xyz;
    out.clip_position = uniforms.view_proj * uniforms.model * vec4(model.position, 1.0);
    return out;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = vec3(1.0, 1.0, 1.0);
    let n_dot_l = max(dot(in.normal, light_dir), 0.0);
    let diffuse = vec3(1.0, 0.0, 0.0);
    let ambient = vec3(0.1, 0.1, 0.1);

    return vec4(ambient + n_dot_l * diffuse, 1.0);
}
