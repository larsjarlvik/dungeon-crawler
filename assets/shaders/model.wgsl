// Vertex shader
struct Uniforms {
    view_proj: mat4x4<f32>;
    model: mat4x4<f32>;
    inv_model: mat4x4<f32>;
    joint_transforms: array<mat4x4<f32>, 64>;
    highlight: f32;
    is_animated: bool;
};

struct PrimitiveUniforms {
    orm_factor: vec4<f32>;
    base_color: vec4<f32>;
    has_textures: bool;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;
[[group(1), binding(0)]] var<uniform> primitive_uniforms: PrimitiveUniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] tangent: vec4<f32>;
    [[location(3)]] tex_coord: vec2<f32>;
    [[location(4)]] weights: vec4<f32>;
    [[location(5)]] joints: vec4<u32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] normal_w: vec3<f32>;
    [[location(2)]] tangent_w: vec3<f32>;
    [[location(3)]] bitangent_w: vec3<f32>;
    [[location(4)]] highlight: f32;
};

[[stage(vertex)]]
fn vert_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var skin_matrix: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
    );

    if (uniforms.is_animated) {
        let w = model.weights;

        for (var i: i32 = 0; i < 4; i = i + 1) {
            let j = model.joints[i];
            var jx: mat4x4<f32> = uniforms.joint_transforms[j];

            skin_matrix = mat4x4<f32>(
                skin_matrix[0] + w[i] * jx[0],
                skin_matrix[1] + w[i] * jx[1],
                skin_matrix[2] + w[i] * jx[2],
                skin_matrix[3] + w[i] * jx[3],
            );
        }
    } else {
        skin_matrix = mat4x4<f32>(
            vec4<f32>(1.0, 0.0, 0.0, 0.0),
            vec4<f32>(0.0, 1.0, 0.0, 0.0),
            vec4<f32>(0.0, 0.0, 1.0, 0.0),
            vec4<f32>(0.0, 0.0, 0.0, 1.0),
        );
    }

    out.normal_w = normalize((uniforms.inv_model * skin_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    out.tangent_w = normalize((uniforms.inv_model * model.tangent).xyz);
    out.bitangent_w = cross(out.normal_w, out.tangent_w) * model.tangent.w;

    out.clip_position = uniforms.view_proj * uniforms.model * skin_matrix * vec4<f32>(model.position, 1.0);
    out.tex_coord = model.tex_coord;
    out.highlight = uniforms.highlight;
    return out;
}

// Fragment shader
[[group(2), binding(0)]] var t_base_color: texture_2d<f32>;
[[group(2), binding(1)]] var t_normal: texture_2d<f32>;
[[group(2), binding(2)]] var t_occlusion_roughness_metallic: texture_2d<f32>;
[[group(2), binding(3)]] var t_sampler: sampler;

[[stage(fragment)]]
fn frag_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color: vec4<f32>;
    var orm: vec4<f32>;
    var tangent: vec4<f32>;
    var normal: vec4<f32>;


    if (primitive_uniforms.has_textures) {
        color = textureSample(t_base_color, t_sampler, in.tex_coord);
        orm = textureSample(t_occlusion_roughness_metallic, t_sampler, in.tex_coord) * primitive_uniforms.orm_factor;

        var tangent: mat3x3<f32> = mat3x3<f32>(in.tangent_w, in.bitangent_w, in.normal_w);
        let normal_texture = textureSample(t_normal, t_sampler, in.tex_coord).xyz;
        normal = vec4<f32>(tangent * normalize(2.0 * normal_texture - 1.0), in.highlight);
    } else {
        color = primitive_uniforms.base_color;
        orm = vec4<f32>(1.0);
        normal = vec4<f32>(in.normal_w, -1.0);
    }

    return color;
}
