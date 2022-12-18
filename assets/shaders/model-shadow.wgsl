// Vertex shader
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    inv_model: mat4x4<f32>,
    joint_transforms: array<mat4x4<f32>, 48>,
    is_animated: u32,
    _padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) weights: vec4<f32>,
    @location(2) joints: vec4<u32>,
}

@vertex
fn vert_main(model: VertexInput) -> @builtin(position) vec4<f32> {
    if (uniforms.is_animated == u32(1)) {
        let w = model.weights;
        var skin_matrix: mat4x4<f32> = mat4x4<f32>(vec4(0.0), vec4(0.0), vec4(0.0), vec4(0.0));

        for (var i: i32 = 0; i < 4; i += 1) {
            skin_matrix += w[i] * uniforms.joint_transforms[model.joints[i]];
        }

        return uniforms.view_proj * uniforms.model * skin_matrix * vec4(model.position, 1.0);
    }

    return uniforms.view_proj * uniforms.model * vec4(model.position, 1.0);
}
