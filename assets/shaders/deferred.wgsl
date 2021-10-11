// Vertex shader
struct Light {
    position: vec3<f32>;
    radius: f32;
    color: vec3<f32>;
};

[[block]]
struct Uniforms {
    inv_view_proj: mat4x4<f32>;
    eye_pos: vec3<f32>;
    viewport_size: vec4<f32>;
    light: array<Light, 32>;
    light_count: i32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

[[stage(vertex)]]
fn main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    let x = i32(vertex_index) / 2;
    let y = i32(vertex_index) & 1;
    let tc = vec2<f32>(f32(x) * 2.0, f32(y) * 2.0);

    return vec4<f32>(
        tc.x - 1.0,
        1.0 - tc.y,
        0.0, 1.0
    );
}

// Fragment shader
var M_PI: f32 = 3.141592653589793;

[[group(1), binding(0)]] var t_depth: texture_2d<f32>;
[[group(1), binding(1)]] var t_normal: texture_2d<f32>;
[[group(1), binding(2)]] var t_color: texture_2d<f32>;
[[group(1), binding(3)]] var t_orm: texture_2d<f32>;

struct PBRInfo {
    n_dot_l: f32;
    n_dot_v: f32;
    n_dot_h: f32;
    l_dot_h: f32;
    v_dot_h: f32;
    roughness: f32;
    metalness: f32;
    reflectance0: vec3<f32>;
    reflectance90: vec3<f32>;
    roughness_sq: f32;
    diffuse: vec3<f32>;
    specular: vec3<f32>;
};

fn world_pos_from_depth(tex_coord: vec2<f32>, depth: f32, inv_matrix: mat4x4<f32>) -> vec3<f32> {
    var ndc: vec3<f32> = vec3<f32>(vec2<f32>(tex_coord.x, 1.0 - tex_coord.y) * 2.0 - 1.0, depth);
    var p: vec4<f32> = inv_matrix * vec4<f32>(ndc, 1.0);
    return p.xyz / p.w;
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
    let roughness_sq2 = pbr.roughness_sq * pbr.roughness_sq;
    let f = (pbr.n_dot_h * roughness_sq2 - pbr.n_dot_h) * pbr.n_dot_h + 1.0;
    return roughness_sq2 / (M_PI * f * f);
}

[[stage(fragment)]]
fn main([[builtin(position)]] coord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    var c: vec2<i32> = vec2<i32>(coord.xy);
    var depth: f32 = textureLoad(t_depth, c, 0).r;

    if (depth >= 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    var color: vec4<f32> = textureLoad(t_color, c, 0);
    var normal: vec3<f32> = normalize(textureLoad(t_normal, c, 0).xyz);
    var orm: vec3<f32> = textureLoad(t_orm, c, 0).xyz;
    var position: vec3<f32> = world_pos_from_depth(coord.xy / uniforms.viewport_size.xy, depth, uniforms.inv_view_proj);

    // PBR
    let view_dir = normalize(uniforms.eye_pos - position);

    var pbr: PBRInfo;
    pbr.roughness = orm.g;
    pbr.metalness = orm.b;
    pbr.roughness_sq = pbr.roughness * pbr.roughness;

    let f0 = vec3<f32>(0.04);
    pbr.diffuse = color.rgb * (vec3<f32>(1.0) - f0);
    pbr.diffuse = pbr.diffuse * (1.0 - pbr.metalness);
    pbr.specular = mix(f0, color.rgb, vec3<f32>(pbr.metalness));
    pbr.reflectance0 = pbr.specular.rgb;
    pbr.reflectance90 = vec3<f32>(1.0) * clamp(max(max(pbr.specular.r, pbr.specular.g), pbr.specular.b) * 5.0, 0.0, 1.0);

    var total_light: vec3<f32> = vec3<f32>(0.1);

    for (var i: i32 = 0; i < uniforms.light_count; i = i + 1) {
        let light = uniforms.light[i];
        var light_dist: f32 = distance(light.position, position);

        if (light_dist < light.radius) {
            let light_dir = normalize(light.position - position);
            let half_dir = normalize(light_dir + view_dir);
            let reflection = -normalize(reflect(view_dir, normal));

            pbr.n_dot_l = clamp(dot(normal, light_dir), 0.001, 1.0);
            pbr.n_dot_v = clamp(abs(dot(normal, view_dir)), 0.001, 1.0);
            pbr.n_dot_h = clamp(dot(normal, half_dir), 0.0, 1.0);
            pbr.l_dot_h = clamp(dot(light_dir, half_dir), 0.0, 1.0);
            pbr.v_dot_h = clamp(dot(view_dir, half_dir), 0.0, 1.0);

            if (pbr.n_dot_l > 0.0 || pbr.n_dot_v > 0.0) {
                var F: vec3<f32> = specularReflection(pbr);
                var G: f32 = geometricOcclusion(pbr);
                var D: f32 = microfacetDistribution(pbr);

                let diffuse_contrib = (1.0 - F) * (pbr.diffuse / M_PI);
                let spec_contrib = F * G * D / (4.0 * pbr.n_dot_l * pbr.n_dot_v);

                let attenuation = clamp(1.0 - light_dist * light_dist / (light.radius * light.radius), 0.0, 1.0);

                var light_contrib: vec3<f32> = (pbr.n_dot_l * (diffuse_contrib + spec_contrib));
                light_contrib = light_contrib + normal.y;

                let new_light = mix(total_light, attenuation * light.color * light_contrib, 0.5);
                total_light = max(total_light, new_light);
            }
        }
    }

    return vec4<f32>(total_light * color.rgb * orm.r, color.a);
}
