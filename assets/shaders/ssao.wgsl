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

let SAMPLE_COUNT: i32 = 32;

[[block]]
struct Uniforms {
    ssao_samples: array<vec4<f32>, SAMPLE_COUNT>;
    viewport: vec4<f32>;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

[[group(1), binding(0)]] var t_depth: texture_2d<f32>;
[[group(1), binding(1)]] var t_sampler: sampler;

fn linearize_depth(d: f32, near: f32, far: f32) -> f32 {
    return near * far / (far + d * (near - far));
}

[[stage(fragment)]]
fn main([[builtin(position)]] coord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    var c: vec2<f32> = vec2<f32>(coord.xy / uniforms.viewport.xy);
    let near = uniforms.viewport.z;
    let far = uniforms.viewport.w;

    let a = linearize_depth(textureSample(t_depth, t_sampler, c).r, near, far);

    var occlusion: f32 = 1.0;
    for (var i: i32 = 0; i < SAMPLE_COUNT; i = i + 1) {
        var sample_pos: vec3<f32> = uniforms.ssao_samples[i].xyz * 0.03;

        let b = linearize_depth(textureSample(t_depth, t_sampler, c + sample_pos.xy).r, near, far);
        if (a > b + 0.002) {
            let range_check = smoothStep(0.0, 1.0, 0.5 / abs(a - b));
            occlusion = (occlusion + 1.0);
        }
    }

    occlusion = 1.0 - occlusion / f32(SAMPLE_COUNT);
    return vec4<f32>(occlusion * 2.0, 0.0, 0.0, 0.0);
}
