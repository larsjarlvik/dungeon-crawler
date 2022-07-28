// Vertex shader
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    inv_model: mat4x4<f32>,
    joint_transforms: array<mat4x4<f32>, 64>,
    highlight: f32,
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
        var skin_matrix: mat4x4<f32> = mat4x4<f32>(
            vec4<f32>(0.0, 0.0, 0.0, 0.0),
            vec4<f32>(0.0, 0.0, 0.0, 0.0),
            vec4<f32>(0.0, 0.0, 0.0, 0.0),
            vec4<f32>(0.0, 0.0, 0.0, 0.0),
        );

        for (var i: i32 = 0; i < 4; i = i + 1) {
            let j = model.joints[i];
            var jx: mat4x4<f32> = uniforms.joint_transforms[j];

            skin_matrix = mat4x4<f32>(
                skin_matrix[0] + w[i] * jx[0],
                skin_matrix[1] + w[i] * jx[1],
                skin_matrix[2] + w[i] * jx[2],
                skin_matrix[3] + w[i] * jx[3],
            );
        }

        return uniforms.view_proj * uniforms.model * skin_matrix * vec4<f32>(model.position, 1.0);
    }

    return uniforms.view_proj * uniforms.model * vec4<f32>(model.position, 1.0);
}
