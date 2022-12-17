// Vertex shader
struct Uniforms {
    position: vec2<f32>,
    size: vec2<f32>,
    background: vec4<f32>,
    background_end: vec4<f32>,
    foreground: vec4<f32>,
    shadow_color: vec4<f32>,
    viewport_size: vec2<f32>,
    shadow_offset: vec2<f32>,
    border_radius: f32,
    shadow_radius: f32,
    opacity: f32,
    gradient_angle: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let x = i32(vertex_index) / 2;
    let y = i32(vertex_index) & 1;
    let tc = vec2(f32(x), f32(y));
    let size = uniforms.size + uniforms.border_radius + uniforms.shadow_radius * 2.0;
    let position = uniforms.position;
    let pos = tc * 2.0 * (size / uniforms.viewport_size) + ((position * 2.0 - 1.0) / uniforms.viewport_size);

    var result: VertexOutput;
    result.coord = tc;
    result.position = vec4(
        pos.x - 1.0,
        1.0 - pos.y,
        0.0, 1.0
    );

    return result;
}

fn round_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2(0.0))) - r;
}

fn sigmoid(t: f32) -> f32 {
    return 1.0 / (1.0 + exp(-t));
}

// Fragment shader
@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let position = in.position.xy + uniforms.shadow_radius * 0.5;
    let hsize = uniforms.size * 0.5;
    let center = uniforms.position + uniforms.size * 0.5;
    var final_color: vec4<f32> = uniforms.background;
    var dist_radius: f32 = 0.0;

    // Border radius
    if (uniforms.border_radius > 0.0 || uniforms.shadow_radius > 0.0) {
        dist_radius = clamp(round_rect(
            position - center,
            hsize,
            uniforms.border_radius),
        0.0, 1.0);
        final_color = vec4(final_color.rgb, final_color.a * (1.0 - dist_radius));
    }

    // Shadows
    if (uniforms.shadow_radius > 0.0) {
        let dist_shadow = clamp(sigmoid(round_rect(
            position - center - uniforms.shadow_offset,
            hsize,
            uniforms.border_radius + uniforms.shadow_radius)),
        0.0, 1.0);

        let shadow_color = vec4(uniforms.shadow_color.rgb, uniforms.shadow_color.a - dist_shadow);
        final_color = mix(final_color, shadow_color, dist_radius);
    }

    // Gradient
    let angle = 1.5708 - uniforms.gradient_angle + atan2(in.coord.y, in.coord.x);
    let grad = cos(angle) * length(in.coord);
    final_color = mix(final_color, uniforms.background_end, smoothstep(0.0, 1.0, grad));

    return vec4(final_color.rgb, final_color.a * uniforms.opacity);
}
