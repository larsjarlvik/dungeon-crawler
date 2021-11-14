
[[block]]
struct Uniforms {
    viewport_width: f32;
    viewport_height: f32;
    has_texture: bool;
};

// Vertex shader
[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] tex_coord: vec2<f32>;
    [[location(2)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(
        model.position.x / uniforms.viewport_width * 2.0 - 1.0,
        model.position.y / uniforms.viewport_height * 2.0 - 1.0,
        0.0,
        1.0
    );

    out.tex_coord = model.tex_coord;
    out.color = model.color;

    return out;
}

// Fragment shader
[[group(1), binding(0)]] var t_texture: texture_2d<f32>;
[[group(1), binding(1)]] var t_sampler: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color: vec4<f32> = in.color;

    if (uniforms.has_texture) {
        color = color * textureSample(t_texture, t_sampler, vec2<f32>(in.tex_coord.x, 1.0 - in.tex_coord.y));
    }

    return color;
}
