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
    joint_transforms: array<mat4x4<f32>, 64>,
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

struct PBRInfo {
    n_dot_l: f32,
    n_dot_v: f32,
    n_dot_h: f32,
    l_dot_h: f32,
    v_dot_h: f32,
    roughness: f32,
    metalness: f32,
    reflectance0: vec3<f32>,
    reflectance90: vec3<f32>,
    roughness_sq: f32,
    roughness_sq2: f32,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
}

fn specularReflection(pbr: PBRInfo) -> vec3<f32> {
    return pbr.reflectance0 + (pbr.reflectance90 - pbr.reflectance0) * pow(clamp(1.0 - pbr.v_dot_h, 0.0, 1.0), 5.0);
}

fn geometricOcclusion(pbr: PBRInfo) -> f32 {
    let r = pbr.roughness_sq;
    let attenuation_l = 2.0 * pbr.n_dot_l / (pbr.n_dot_l + sqrt(r * r + (1.0 - r * r) * (pbr.n_dot_l * pbr.n_dot_l)));
    let attenuation_v = 2.0 * pbr.n_dot_v / (pbr.n_dot_v + sqrt(r * r + (1.0 - r * r) * (pbr.n_dot_v * pbr.n_dot_v)));
    return attenuation_l * attenuation_v;
}

fn microfacetDistribution(pbr: PBRInfo) -> f32 {
    let f = (pbr.n_dot_h * pbr.roughness_sq2 - pbr.n_dot_h) * pbr.n_dot_h + 1.0;
    return pbr.roughness_sq2 / (M_PI * f * f);
}

fn contrast_matrix(contrast: f32) -> mat4x4<f32> {
    let t = (1.0 - contrast) * 0.5;

    return mat4x4<f32>(
        vec4<f32>(contrast, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, contrast, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, contrast, 0.0),
        vec4<f32>(t, t, t, 1.0)
    );
}

fn attenuation_strength_real(rpos: vec3<f32>) -> f32 {
    let d2 = rpos.x * rpos.x + rpos.y * rpos.y + rpos.z * rpos.z;
    return 1.0 / (0.025 + d2);
}

fn apply_point_glow(wpos: vec3<f32>, dir: vec3<f32>, max_dist: f32, position: vec3<f32>, bloom: f32) -> f32 {
    let t = max(dot(position - wpos, dir), 0.0);
    let nearest = wpos + dir * min(t, max_dist);

    let difference = position - nearest;
    let spread = 1.0;
    let strength = pow(attenuation_strength_real(difference), spread); // TODO
    return strength * 0.025 * pow(bloom, 0.65);
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32>;
    var orm: vec4<f32>;
    var tangent: vec4<f32>;
    var normal: vec4<f32>;
    var normal_t: vec4<f32>;


    if (primitive_uniforms.has_textures == u32(1)) {
        color = textureSample(t_base_color, t_sampler, in.tex_coord);
        orm = textureSample(t_occlusion_roughness_metallic, t_sampler, in.tex_coord) * primitive_uniforms.orm_factor;

        var tangent: mat3x3<f32> = mat3x3<f32>(in.tangent_w, in.bitangent_w, in.normal_w);
        normal_t = textureSample(t_normal, t_sampler, in.tex_coord);
        normal = vec4<f32>(tangent * normalize(2.0 * normal_t.xyz - 1.0), in.highlight);
    } else {
        return primitive_uniforms.base_color;
    }

    // PBR
    let position = in.world_position.xyz;
    let view_dir = normalize(env_uniforms.eye_pos - position);

    var pbr: PBRInfo;
    pbr.roughness = orm.g;
    pbr.metalness = orm.b * 2.0;
    pbr.roughness_sq = pbr.roughness * pbr.roughness;
    pbr.roughness_sq2 = pbr.roughness_sq * pbr.roughness_sq;

    let f0 = vec3<f32>(0.05);
    pbr.diffuse = color.rgb * (vec3<f32>(1.0) - f0);
    pbr.diffuse = pbr.diffuse * (1.0 - pbr.metalness);
    pbr.specular = mix(f0, color.rgb, vec3<f32>(pbr.metalness));
    pbr.reflectance0 = pbr.specular.rgb;
    pbr.reflectance90 = vec3<f32>(1.0) * clamp(max(max(pbr.specular.r, pbr.specular.g), pbr.specular.b) * 5.0, 0.0, 1.0);

    var total_light: vec3<f32> = vec3<f32>(smoothstep(0.1, 0.0, distance(env_uniforms.eye_target, position) * 0.015) * 0.1);

    pbr.n_dot_v = clamp(abs(dot(normal.xyz, view_dir)), 0.001, 1.0);
    let reflection = -normalize(reflect(view_dir, normal.xyz));

    for (var i: i32 = 0; i < env_uniforms.light_count; i = i + 1) {
        let light = env_uniforms.light[i];
        let light_dist = distance(light.position, position);
        if (light_dist > light.radius) { continue; }

        let attenuation = smoothstep(light.radius, 0.0, light_dist);
        let light_dir = normalize(light.position - position);
        let half_dir = normalize(light_dir + view_dir);

        pbr.n_dot_l = clamp(dot(normal.xyz, light_dir), 0.001, 1.0);
        pbr.n_dot_h = clamp(dot(normal.xyz, half_dir), 0.0, 1.0);
        pbr.l_dot_h = clamp(dot(light_dir, half_dir), 0.0, 1.0);
        pbr.v_dot_h = clamp(dot(view_dir, half_dir), 0.0, 1.0);

        if (pbr.n_dot_l > 0.0 || pbr.n_dot_v > 0.0) {
            let F = specularReflection(pbr);
            let G = geometricOcclusion(pbr);
            let D = microfacetDistribution(pbr);

            let diffuse_contrib = (1.0 - F) * (pbr.diffuse / M_PI);
            let spec_contrib = F * G * D / (4.0 * pbr.n_dot_l * pbr.n_dot_v);

            let light_contrib = (pbr.n_dot_l * (diffuse_contrib + spec_contrib));
            let new_light = attenuation * light.color * light_contrib;


            total_light = total_light + new_light;

            if (light.bloom > 0.1) {
                let dist = distance(position, env_uniforms.eye_pos.xyz);
                let dir = (position - env_uniforms.eye_pos.xyz) / dist;
                let bloom = light.color * apply_point_glow(env_uniforms.eye_pos, dir, dist, light.position, light.bloom);
                total_light = total_light + bloom;
            }
        }
    }

    return contrast_matrix(env_uniforms.contrast) * vec4<f32>(total_light * color.rgb * orm.r, color.a);
}
