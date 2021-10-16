// Vertex shader
[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
    model: mat4x4<f32>;
    joint_transforms: array<mat4x4<f32>, 64>;
    is_animated: bool;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] weights: vec4<f32>;
    [[location(2)]] joints: vec4<u32>;
};

[[stage(vertex)]]
fn main(model: VertexInput) -> [[builtin(position)]] vec4<f32> {
    var skin_matrix: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 0.0),
    );

    if (uniforms.is_animated) {
        let w = model.weights;

        for (var i: i32 = 0; i < 4; i = i + 1) {
            let j = model.joints[i];
            var jx: mat4x4<f32> = uniforms.joint_transforms[j];

            skin_matrix = mat4x4<f32>(
                vec4<f32>(skin_matrix[0][0] + w[i] * jx[0][0], skin_matrix[0][1] + w[i] * jx[0][1], skin_matrix[0][2] + w[i] * jx[0][2], skin_matrix[0][3] + w[i] * jx[0][3]),
                vec4<f32>(skin_matrix[1][0] + w[i] * jx[1][0], skin_matrix[1][1] + w[i] * jx[1][1], skin_matrix[1][2] + w[i] * jx[1][2], skin_matrix[1][3] + w[i] * jx[1][3]),
                vec4<f32>(skin_matrix[2][0] + w[i] * jx[2][0], skin_matrix[2][1] + w[i] * jx[2][1], skin_matrix[2][2] + w[i] * jx[2][2], skin_matrix[2][3] + w[i] * jx[2][3]),
                vec4<f32>(skin_matrix[3][0] + w[i] * jx[3][0], skin_matrix[3][1] + w[i] * jx[3][1], skin_matrix[3][2] + w[i] * jx[3][2], skin_matrix[3][3] + w[i] * jx[3][3]),
            );
        }
    } else {
        skin_matrix = mat4x4<f32>(
            vec4<f32>(1.0, 0.0, 0.0, 0.0),
            vec4<f32>(0.0, 1.0, 0.0, 0.0),
            vec4<f32>(0.0, 0.0, 1.0, 0.0),
            vec4<f32>(0.0, 0.0, 0.0, 1.0),
        );
    }

    return uniforms.view_proj * uniforms.model * skin_matrix * vec4<f32>(model.position, 1.0);
}
