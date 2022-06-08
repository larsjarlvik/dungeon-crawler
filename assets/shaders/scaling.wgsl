// Vertex shader
[[stage(vertex)]]
fn vert_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
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
    viewport: vec2<f32>;
    sharpen: bool;
    scale: f32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

[[group(1), binding(0)]] var t_texture: texture_2d<f32>;
[[group(1), binding(1)]] var t_sampler: sampler;

fn sharp(pix_coord: f32) -> f32 {
    let norm = (fract(pix_coord) - 0.5) * 2.0;
    let norm2 = norm * norm;
    return floor(pix_coord) + norm * pow(norm2, 2.0) / 2.0 + 0.5;
}

[[stage(fragment)]]
fn frag_main([[builtin(position)]] coord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    if (!uniforms.sharpen) {
        return textureSample(t_texture, t_sampler, coord.xy / uniforms.viewport);
    }

    let pos = vec2<f32>(sharp(coord.x * uniforms.scale) / uniforms.scale, sharp(coord.y * uniforms.scale) / uniforms.scale);
    return textureSample(t_texture, t_sampler, pos / uniforms.viewport);
}
