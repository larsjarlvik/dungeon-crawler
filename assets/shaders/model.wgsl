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
    is_animated: u32,
    _padding: vec2<f32>,
}

struct EnvironmentUniforms {
    eye_pos: vec3<f32>,
    eye_target: vec3<f32>,
    light: array<Light, 20>,
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
    @location(5) world_position: vec4<f32>,
}

@vertex
fn vert_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    if (uniforms.is_animated == u32(1)) {
        let w = model.weights;
        var skin_matrix: mat4x4<f32> = mat4x4<f32>(vec4(0.0), vec4(0.0), vec4(0.0), vec4(0.0));

        for (var i: i32 = 0; i < 4; i += 1) {
            skin_matrix += w[i] * uniforms.joint_transforms[model.joints[i]];
        }

        out.world_position = uniforms.model * skin_matrix * vec4(model.position, 1.0);
    } else {
        out.world_position = uniforms.model * vec4(model.position, 1.0);
    }

    out.normal_w = normalize((uniforms.inv_model * vec4(model.normal, 0.0)).xyz);
    out.tangent_w = normalize((uniforms.inv_model * model.tangent).xyz);
    out.bitangent_w = cross(out.normal_w, out.tangent_w) * model.tangent.w;

    out.clip_position = uniforms.view_proj * out.world_position;
    out.tex_coord = model.tex_coord;
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
        vec4(contrast, 0.0, 0.0, 0.0),
        vec4(0.0, contrast, 0.0, 0.0),
        vec4(0.0, 0.0, contrast, 0.0),
        vec4(t, t, t, 1.0)
    );
}

fn attenuation_strength_real(rpos: vec3<f32>) -> f32 {
    let d2 = rpos.x * rpos.x + rpos.y * rpos.y + rpos.z * rpos.z;
    return 1.0 / (0.005 + d2);
}

fn apply_point_glow(wpos: vec3<f32>, dir: vec3<f32>, max_dist: f32, position: vec3<f32>, bloom: f32) -> f32 {
    let t = max(dot(position - wpos, dir), 0.0);
    let nearest = wpos + dir * min(t, max_dist);

    let difference = position - nearest;
    let spread = 1.0;
    let strength = pow(attenuation_strength_real(difference), spread); // TODO
    return strength * 0.005 * pow(bloom, 0.65);
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

fn geometry_smith(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
    let ggx2 = gometry_schlick_ggx(n_dot_v, roughness);
    let ggx1 = gometry_schlick_ggx(n_dot_l, roughness);

    return ggx1 * ggx2;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (primitive_uniforms.has_textures == u32(0)) {
        return primitive_uniforms.base_color;
    }

    let albedo = pow(textureSample(t_base_color, t_sampler, in.tex_coord), vec4(2.2));
    let orm = pow(textureSample(t_occlusion_roughness_metallic, t_sampler, in.tex_coord), vec4(2.2));
    let occlusion = orm.r;
    let roughness = orm.g;
    let metalness = orm.b;

    let tangent: mat3x3<f32> = mat3x3<f32>(in.tangent_w, in.bitangent_w, in.normal_w);
    let normal_t = textureSample(t_normal, t_sampler, in.tex_coord);
    let normal = tangent * normalize(2.0 * normal_t.xyz - 1.0);

    // PBR
    let position = in.world_position.xyz;
    let view_dir = normalize(env_uniforms.eye_pos - position);

    let f0 = mix(vec3(0.04), albedo.rgb, metalness);
    var lo = vec3(0.0);

    for (var i: i32 = 0; i < env_uniforms.light_count; i += 1) {
        let light = env_uniforms.light[i];
        let light_dist = distance(light.position, position);
        if (light_dist > light.radius) { continue; }

        let attenuation = pow(clamp(pow(1.0 - (light_dist / light.radius), 2.0), 0.0, 1.0), 2.0) / light_dist * light_dist;
        let radiance = light.color * attenuation;

        let light_dir = normalize(light.position - position);
        let half_dir = normalize(view_dir + light_dir);
        let n_dot_v = max(dot(normal, view_dir), 0.0);
        let n_dot_l = max(dot(normal, light_dir), 0.0);

        let ndf = distribution_ggx(normal, half_dir, roughness);
        let g = geometry_smith(n_dot_v, n_dot_l, roughness);
        let f = fresnel_schlick(max(dot(half_dir, view_dir), 0.0), f0);

        let kd = (vec3(1.0) - f) * (1.0 - metalness);
        let numerator = ndf * g * f;
        let denominator = 4.0 * max(dot(normal, view_dir), 0.0) * max(dot(normal, light_dir), 0.0) + 0.0001;
        let specular = numerator / denominator;

        lo += (kd * albedo.rgb / M_PI + specular) * radiance * n_dot_l;

        // Reflections
        let reflect_dir = reflect(-light_dir, normal);
        let reflection = pow(max(dot(view_dir, reflect_dir), 0.0), 48.0);
        lo += metalness * (1.0 - roughness) * reflection * light.color;

        if (light.bloom > 0.1) {
            let dir = (position - env_uniforms.eye_pos.xyz) / light_dist;
            let bloom = vec3(apply_point_glow(env_uniforms.eye_pos, dir, light_dist, light.position, light.bloom));
            lo += bloom * denominator;
        }
    }

    var color: vec3<f32> = lo / (lo + vec3(1.0)) * occlusion;
    color = pow(color, vec3(1.0 / 1.8));

    return contrast_matrix(env_uniforms.contrast) * vec4(color, albedo.a);
}
