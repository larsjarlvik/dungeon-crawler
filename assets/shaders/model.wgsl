// Vertex shader
struct Light {
    position: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    bloom: f32,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    inv_model: mat4x4<f32>,
    joint_transforms: array<mat4x4<f32>, 48>,
    highlight: f32,
    is_animated: u32,
    _padding: vec2<f32>,
}

struct EnvironmentUniforms {
    eye_pos: vec3<f32>,
    eye_target: vec3<f32>,
    light: array<Light, 16>,
    light_count: i32,
    contrast: f32,
}

struct PrimitiveUniforms {
    orm_factor: vec4<f32>,
    base_color: vec4<f32>,
    has_textures: u32,
    _padding: vec3<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var<uniform> env_uniforms: EnvironmentUniforms;
@group(2) @binding(0) var<uniform> primitive_uniforms: PrimitiveUniforms;

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
    @location(0) tex_coord: vec2<f32>,
    @location(1) normal_w: vec3<f32>,
    @location(2) tangent_w: vec3<f32>,
    @location(3) bitangent_w: vec3<f32>,
    @location(4) highlight: f32,
    @location(5) world_position: vec4<f32>,
}

@vertex
fn vert_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var skin_matrix: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
    );

    if (uniforms.is_animated == u32(1)) {
        let w = model.weights;

        for (var i: i32 = 0; i < 4; i += 1) {
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

    out.normal_w = normalize((uniforms.inv_model * vec4<f32>(model.normal, 0.0)).xyz);
    out.tangent_w = normalize((uniforms.inv_model * model.tangent).xyz);
    out.bitangent_w = cross(out.normal_w, out.tangent_w) * model.tangent.w;

    out.world_position = uniforms.model * skin_matrix * vec4<f32>(model.position, 1.0);
    out.clip_position = uniforms.view_proj * out.world_position;
    out.tex_coord = model.tex_coord;
    out.highlight = uniforms.highlight;
    return out;
}

// Fragment shader
let M_PI = 3.141592653589793;

@group(3) @binding(0) var t_base_color: texture_2d<f32>;
@group(3) @binding(1) var t_normal: texture_2d<f32>;
@group(3) @binding(2) var t_occlusion_roughness_metallic: texture_2d<f32>;
@group(3) @binding(3) var t_sampler: sampler;

fn contrast_matrix(contrast: f32) -> mat4x4<f32> {
    let t = (1.0 - contrast) * 0.5;

    return mat4x4<f32>(
        vec4<f32>(contrast, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, contrast, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, contrast, 0.0),
        vec4<f32>(t, t, t, 1.0)
    );
}

fn fresnel_schlick(cosTheta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (1.0 - f0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn distribution_ggx(N: vec3<f32>, half_dir: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, half_dir), 0.0);
    let NdotH2 = NdotH * NdotH;

    let num = a2;
    var denom: f32 = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = M_PI * denom * denom;

    return num / denom;
}

fn gometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;

    let num = n_dot_v;
    let denom = n_dot_v * (1.0 - k) + k;

    return num / denom;
}

fn geometry_smith(N: vec3<f32>, V: vec3<f32>, light_dir: vec3<f32>, roughness: f32) -> f32 {
    let n_dot_v = max(dot(N, V), 0.0);
    let n_dot_l = max(dot(N, light_dir), 0.0);
    let ggx2 = gometry_schlick_ggx(n_dot_v, roughness);
    let ggx1 = gometry_schlick_ggx(n_dot_l, roughness);

    return ggx1 * ggx2;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var albedo: vec4<f32>;
    var occlusion: f32;
    var roughness: f32;
    var metalness: f32;
    var tangent: vec4<f32>;
    var normal: vec3<f32>;
    var normal_t: vec4<f32>;


    if (primitive_uniforms.has_textures == u32(1)) {
        albedo = pow(textureSample(t_base_color, t_sampler, in.tex_coord), vec4<f32>(2.2));
        let orm = pow(textureSample(t_occlusion_roughness_metallic, t_sampler, in.tex_coord), vec4<f32>(2.2));
        occlusion = orm.r;
        roughness = orm.g;
        metalness = orm.b;

        var tangent: mat3x3<f32> = mat3x3<f32>(in.tangent_w, in.bitangent_w, in.normal_w);
        normal_t = textureSample(t_normal, t_sampler, in.tex_coord);
        normal = tangent * normalize(2.0 * normal_t.xyz - 1.0);
    } else {
        return primitive_uniforms.base_color;
    }

    // PBR
    let position = in.world_position.xyz;
    let view_dir = normalize(env_uniforms.eye_pos - position);

    let f0 = mix(vec3<f32>(0.04), albedo.rgb, metalness);
    var lo = vec3<f32>(0.0);

    for (var i: i32 = 0; i < env_uniforms.light_count; i += 1) {
        let light = env_uniforms.light[i];
        let light_dist = distance(light.position, position);
        if (light_dist > light.radius) { continue; }

        let distance = length(light.position - position);

        let light_dir = normalize(light.position - position);
        let half_dir = normalize(view_dir + light_dir);
        let attenuation = clamp(1.0 - distance * distance / (light.radius * light.radius), 0.0, 1.0);
        let attenuation = attenuation * attenuation;


        let radiance = light.color * attenuation;

        let ndf = distribution_ggx(normal, half_dir, roughness);
        let g = geometry_smith(normal, view_dir, light_dir, roughness);
        let f = fresnel_schlick(max(dot(half_dir, view_dir), 0.0), f0);

        let kd = (vec3<f32>(1.0) - f) * (1.0 - metalness);
        let numerator = ndf * g * f;
        let denominator = 4.0 * max(dot(normal, view_dir), 0.0) * max(dot(normal, light_dir), 0.0) + 0.0001;
        let specular = numerator / denominator;

        let n_dot_l = max(dot(normal, light_dir), 0.0);
        lo += (kd * albedo.rgb / M_PI + specular) * radiance * n_dot_l;

        // TODO: bloom?
    }

    let ambient = vec3<f32>(0.0) * albedo.rgb * occlusion;
    var color: vec3<f32> = ambient + lo * in.highlight;
    color = color / (color + vec3(1.0));
    color = pow(color, vec3<f32>(1.0 / 2.0));

    return contrast_matrix(env_uniforms.contrast) * vec4<f32>(color, albedo.a);
}
