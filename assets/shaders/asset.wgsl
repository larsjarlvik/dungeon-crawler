// Vertex shader
struct Uniforms {
    position: vec2<f32>;
    size: vec2<f32>;
    background: vec4<f32>;
    viewport_size: vec2<f32>;
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

[[stage(fragment)]]
fn frag_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    if (uniforms.has_image == false) {
        return uniforms.background;
    }

    return uniforms.background * textureSample(t_texture, t_sampler, in.coord);
}
