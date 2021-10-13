// Vertex shader
[[stage(vertex)]]
fn main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
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
[[block]]
struct Uniforms {
    width: f32;
    height: f32;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

[[group(1), binding(0)]] var t_texture: texture_2d<f32>;
[[group(1), binding(1)]] var t_sampler: sampler;

[[stage(fragment)]]
fn main([[builtin(position)]] coord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    return textureSample(t_texture, t_sampler, coord.xy / vec2<f32>(uniforms.width, uniforms.height));
}
