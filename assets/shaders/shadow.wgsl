
struct Uniforms {
    shadow_matrix: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    viewport_size: vec4<f32>,
    shadow_size: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(1) @binding(0) var t_depth: texture_2d<f32>;
@group(1) @binding(1) var t_color: texture_2d<f32>;
@group(1) @binding(2) var t_shadow: texture_depth_2d;
@group(1) @binding(3) var t_shadow_sampler: sampler_comparison;

@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = i32(vertex_index) / 2;
    let y = i32(vertex_index) & 1;
    let tc = vec2<f32>(f32(x) * 2.0, f32(y) * 2.0);

    return vec4<f32>(
        tc.x - 1.0,
        1.0 - tc.y,
        0.0, 1.0
    );
}

fn world_pos_from_depth(tex_coord: vec2<f32>, depth: f32, inv_matrix: mat4x4<f32>) -> vec3<f32> {
    var ndc: vec3<f32> = vec3<f32>(vec2<f32>(tex_coord.x, 1.0 - tex_coord.y) * 2.0 - 1.0, depth);
    var p: vec4<f32> = inv_matrix * vec4<f32>(ndc, 1.0);
    return p.xyz / p.w;
}

let min_shadow = 0.2;
let disk_size = 12;

@fragment
fn frag_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    var disk: array<vec2<f32>, disk_size> = array<vec2<f32>, disk_size>(
        vec2<f32>(-.326,-.406),
        vec2<f32>(-.840,-.074),
        vec2<f32>(-.696, .457),
        vec2<f32>(-.203, .621),
        vec2<f32>( .962,-.195),
        vec2<f32>( .473,-.480),
        vec2<f32>( .519, .767),
        vec2<f32>( .185,-.893),
        vec2<f32>( .507, .064),
        vec2<f32>( .896, .412),
        vec2<f32>(-.322,-.933),
        vec2<f32>(-.792,-.598),
    );

    var c: vec2<i32> = vec2<i32>(coord.xy);
    var depth: f32 = textureLoad(t_depth, c, 0).r;
    let color = textureLoad(t_color, c, 0);

    let position = world_pos_from_depth(coord.xy / uniforms.viewport_size.xy, depth, uniforms.inv_view_proj);
    var shadow_coords: vec4<f32> = uniforms.shadow_matrix * vec4<f32>(position, 1.0);
    if (shadow_coords.w <= 0.0) {
        return color;
    }

    let flip_correction = vec2<f32>(0.5, -0.5);
    let proj_correction = 1.0 / shadow_coords.w;
    let light_local = shadow_coords.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5);

    let shadow_scale = 1.6 / uniforms.shadow_size;
    var shadow_factor = 0.0;

    for (var i: i32 = 0; i < disk_size; i += 1) {
        let offset = disk[i] * shadow_scale;
        shadow_factor += textureSampleCompareLevel(t_shadow, t_shadow_sampler, light_local + offset, (shadow_coords.z - 0.03) * proj_correction);
    }

    let shadow = shadow_factor / f32(disk_size) * (1.0 - min_shadow) + min_shadow;
    return vec4<f32>(color.rgb * shadow, color.a);
}
