// Vertex shader
struct Uniforms {
    position: vec2<f32>;
    size: vec2<f32>;
    background: vec4<f32>;
    background_end: vec4<f32>;
    foreground: vec4<f32>;
    shadow_color: vec4<f32>;
    viewport_size: vec2<f32>;
    shadow_offset: vec2<f32>;
    border_radius: f32;
    shadow_radius: f32;
    opacity: f32;
    has_image: bool;
    gradient_angle: f32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] coord: vec2<f32>;
};

[[stage(vertex)]]
fn vert_main([[builtin(vertex_index)]] vertex_index: u32) -> VertexOutput {
    let x = i32(vertex_index) / 2;
    let y = i32(vertex_index) & 1;
    let tc = vec2<f32>(f32(x), f32(y));

    var result: VertexOutput;
    result.coord = tc;

    let size = uniforms.size + uniforms.shadow_radius * 10.0;
    let position = uniforms.position - uniforms.shadow_radius * 5.0;

    let pos = tc * 2.0 * (size / uniforms.viewport_size) + ((position * 2.0 - 1.0) / uniforms.viewport_size);
    result.position = vec4<f32>(
        pos.x - 1.0,
        1.0 - pos.y,
        0.0, 1.0
    );
    return result;
}

fn round_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - r;
}

fn normal_blend(src: vec4<f32>, dst: vec4<f32>) -> vec4<f32> {
    let final_alpha = src.a + dst.a * (1.0 - src.a);
    return vec4<f32>(
        (src.rgb * src.a + dst.rgb * dst.a * (1.0 - src.a)) / final_alpha,
        final_alpha
    );
}

fn sigmoid(t: f32) -> f32 {
    return 1.0 / (1.0 + exp(-t));
}


// Fragment shader
[[group(1), binding(0)]] var t_texture: texture_2d<f32>;
[[group(1), binding(1)]] var t_sampler: sampler;

[[stage(fragment)]]
fn frag_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    if (uniforms.has_image == false) {
        let angle = 1.5708 - uniforms.gradient_angle + atan2(in.coord.y, in.coord.x);
        var final_color: vec4<f32> = uniforms.background;

        if (uniforms.shadow_radius > 0.0 || uniforms.border_radius > 0.0) {
            let size = uniforms.size;
            let position = in.position.xy + uniforms.shadow_radius * 0.5;

            let shadow_radius = uniforms.shadow_radius;
            let center = uniforms.position + shadow_radius + size * 0.5;
            let hsize = size * 0.5 - uniforms.border_radius * 0.1;

            let dist_shadow = clamp(sigmoid(round_rect(position - center - uniforms.shadow_offset, hsize, uniforms.border_radius + shadow_radius) / shadow_radius), 0.0, 1.0);
            let dist_radius = clamp(round_rect(position - center, hsize, uniforms.border_radius), 0.0, 1.0);

            let shadow_color = vec4<f32>(uniforms.shadow_color.rgb, uniforms.shadow_color.a - dist_shadow);
            let element_color = vec4<f32>(final_color.rgb, final_color.a * (1.0 - dist_radius));
            final_color = mix(element_color, shadow_color, dist_radius);
        }

        let grad = cos(angle) * length(in.coord);
        final_color = mix(final_color, uniforms.background_end, smoothStep(0.0, 1.0, grad));

        return vec4<f32>(final_color.rgb, final_color.a * uniforms.opacity);
    }

    let texture = textureSample(t_texture, t_sampler, in.coord);
    return vec4<f32>(mix(texture.rgb, uniforms.foreground.rgb, uniforms.foreground.a), texture.a * uniforms.opacity);
}
