// Vertex shader
struct Light {
    position: vec3<f32>;
    attenuation: f32;
    direction: vec3<f32>;
    directional: bool;
    color: vec3<f32>;
};

[[block]]
struct Uniforms {
    inv_view_proj: mat4x4<f32>;
    eye_pos: vec3<f32>;
    viewport_size: vec4<f32>;
    light: array<Light, 10>;
    light_count: i32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

var positions: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>( 1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0,-1.0),
    vec2<f32>(-1.0,-1.0),
    vec2<f32>( 1.0,-1.0),
    vec2<f32>( 1.0, 1.0)
);

[[stage(vertex)]]
fn main([[builtin(vertex_index)]] in_vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(positions[in_vertex_index], 0.0, 1.0);
}

// Fragment shader
[[group(1), binding(0)]] var t_depth: texture_2d<f32>;
[[group(1), binding(1)]] var t_normal: texture_2d<f32>;
[[group(1), binding(2)]] var t_color: texture_2d<f32>;

fn world_pos_from_depth(tex_coord: vec2<f32>, depth: f32, inv_matrix: mat4x4<f32>) -> vec3<f32> {
    var ndc: vec3<f32> = vec3<f32>(vec2<f32>(tex_coord.x, 1.0 - tex_coord.y) * 2.0 - 1.0, depth);
    var p: vec4<f32> = inv_matrix * vec4<f32>(ndc, 1.0);
    return p.xyz / p.w;
}

[[stage(fragment)]]
fn main([[builtin(position)]] coord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    var c: vec2<i32> = vec2<i32>(coord.xy);
    var depth: f32 = textureLoad(t_depth, c, 0).r;
    var color: vec4<f32> = textureLoad(t_color, c, 0);
    var normal: vec3<f32> = textureLoad(t_normal, c, 0).xyz;

    if (depth >= 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    var position: vec3<f32> = world_pos_from_depth(coord.xy / uniforms.viewport_size.xy, depth, uniforms.inv_view_proj);
    var view_dir: vec3<f32> = normalize(uniforms.eye_pos - position);
    var light: vec3<f32> = vec3<f32>(0.2);

    for (var i: i32 = 0; i < uniforms.light_count; i = i + 1) {
        var direction: vec3<f32> = normalize(uniforms.light[i].position - position);
        var half_dir: vec3<f32> = normalize(view_dir - direction);

        let diffuse = max(dot(normal, -direction), 0.0);
        let specular = pow(max(dot(normal, half_dir), 0.0), 10.0) * 4.0;

        light = light + uniforms.light[i].color * (diffuse + specular);
    }

    return vec4<f32>(light * color.rgb, color.a);
}
