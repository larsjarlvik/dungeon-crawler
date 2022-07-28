// Vertex shader
@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = i32(vertex_index) / 2;
    let y = i32(vertex_index) & 1;
    let tc = vec2<f32>(f32(x) * 2.0, f32(y) * 2.0);

    return vec4<f32>(
        tc.x * 2.0 - 1.0,
        1.0 - tc.y * 2.0,
        0.0, 1.0
    );
}

// Fragment shader
struct Uniforms {
    viewport: vec2<f32>,
    sharpen: u32,
    scale: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(1) @binding(0) var t_texture: texture_2d<f32>;
@group(1) @binding(1) var t_sampler: sampler;

@fragment
fn frag_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = coord.xy / uniforms.viewport;
    let col = textureSample(t_texture, t_sampler, uv);

    if (uniforms.sharpen == u32(0)) {
        return col;
    }

    let scaled_size = uniforms.viewport / uniforms.scale;

    var max_g: f32 = col.y;
    var min_g: f32 = col.y;
    var uvoff: vec4<f32> = vec4<f32>(1.0, 1.0, -1.0, -1.0) / vec4<f32>(scaled_size.x, scaled_size.x, scaled_size.y, scaled_size.y);
    var col1: vec3<f32> = textureSample(t_texture, t_sampler, uv+uvoff.yw).xyz;
    max_g = max(max_g, col1.y);
    min_g = min(min_g, col1.y);
    var colw: vec3<f32> = col1;
    col1 = textureSample(t_texture, t_sampler, uv + uvoff.xy).xyz;
    max_g = max(max_g, col1.y);
    min_g = min(min_g, col1.y);
    colw = colw + col1;
    col1 = textureSample(t_texture, t_sampler, uv + uvoff.yz).xyz;
    max_g = max(max_g, col1.y);
    min_g = min(min_g, col1.y);
    colw = colw + col1;
    col1 = textureSample(t_texture, t_sampler, uv - uvoff.xy).xyz;
    max_g = max(max_g, col1.y);
    min_g = min(min_g, col1.y);
    colw = colw + col1;

    let d_min_g = min_g;
    let d_max_g = 1.0 - max_g;
    var amp: f32;
    let max_g = max(0.0 , max_g);
    if (d_max_g < d_min_g) {
        amp = d_max_g / max_g;
    } else {
        amp = d_min_g / max_g;
    }
    amp = sqrt(max(0.0, amp)) * -0.18;
    let col_out = (col.xyz + colw * vec3<f32>(amp)) / vec3<f32>(1.0 + 4.0 * amp);

    return vec4<f32>(col_out, 1.0);
}
