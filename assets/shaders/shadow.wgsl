
struct Uniforms {
    shadow_inv_vp: mat4x4<f32>,
    inv_vp_w: vec4<f32>,
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
    let tc = vec2(f32(x) * 2.0, f32(y) * 2.0);

    return vec4(
        tc.x - 1.0,
        1.0 - tc.y,
        0.0, 1.0
    );
}

const min_shadow = 0.2;
const disk_size = 4;

@fragment
fn frag_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    var disk: array<vec2<f32>, disk_size> = array<vec2<f32>, disk_size>(
        vec2(-0.840,-0.074),
        vec2(-0.203, 0.621),
        vec2( 0.473,-0.480),
        vec2( 0.896, 0.412),
    );
    var c: vec2<i32> = vec2<i32>(coord.xy);
    let depth = textureLoad(t_depth, c, 0).r;
    let color = textureLoad(t_color, c, 0);

    let tex_coord = coord.xy / uniforms.viewport_size.xy;
    let ndc = vec4(vec2(tex_coord.x, 1.0 - tex_coord.y) * 2.0 - 1.0, depth, 1.0);

    let shadow_coords = uniforms.shadow_inv_vp * ndc / dot(uniforms.inv_vp_w, ndc);
    let flip_correction = vec2(0.5, -0.5);
    let light_local = shadow_coords.xy * flip_correction + vec2(0.5, 0.5);

    let shadow_scale = 1.0 / uniforms.shadow_size;
    var shadow_factor = 0.0;

    for (var i: i32 = 0; i < disk_size; i += 1) {
        let offset = disk[i] * shadow_scale;
        shadow_factor += textureSampleCompareLevel(t_shadow, t_shadow_sampler, light_local + offset, shadow_coords.z - 0.003);
    }

    let shadow = shadow_factor * 0.25 * (1.0 - min_shadow) + min_shadow;
    return vec4(color.rgb * shadow, color.a);
}
