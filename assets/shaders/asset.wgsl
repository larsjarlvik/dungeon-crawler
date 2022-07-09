// Vertex shader
struct Uniforms {
    position: vec2<f32>;
    size: vec2<f32>;
    background: vec4<f32>;
    foreground: vec4<f32>;
    viewport_size: vec2<f32>;
    variant: u32;
    opacity: f32;
    has_image: bool;
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

    let tc_pos = tc * 2.0 * (uniforms.size / uniforms.viewport_size) + ((uniforms.position * 2.0 - 1.0) / uniforms.viewport_size);
    result.position = vec4<f32>(
        tc_pos.x - 1.0,
        1.0 - tc_pos.y,
        0.0, 1.0
    );
    return result;
}

// Fragment shader
[[group(1), binding(0)]] var t_texture: texture_2d<f32>;
[[group(1), binding(1)]] var t_sampler: sampler;

fn rounded(pos: vec2<f32>, size: vec2<f32>, radius: f32, thickness: f32) -> f32 {
    let d = length(max(pos, size) - size) - radius;
    return smoothStep(0.55, 0.45, abs(d / thickness) * 5.0);
}

[[stage(fragment)]]
fn frag_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    if (uniforms.has_image == false) {
        var current: f32 = 1.0;

        if (uniforms.variant == u32(1)) {
            let coord = in.coord * 2.0 - 1.0;
            let fade = (1.0 - length(coord)) + 0.3;
            let outer = step(length(coord), 1.0);
            let inner = step(length(coord), 0.95) * fade;
            current = clamp(outer - inner, 0.0, 1.0);
        }

        return vec4<f32>(uniforms.background.rgb, uniforms.background.a * uniforms.opacity * current);
    }

    let texture = textureSample(t_texture, t_sampler, in.coord);
    return vec4<f32>(mix(texture.rgb, uniforms.foreground.rgb, uniforms.foreground.a), texture.a * uniforms.opacity);
}
