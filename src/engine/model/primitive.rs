use super::vertex::Vertex;

pub struct Primitive {
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
    pub material: usize,
    pub length: u32,
}

impl Primitive {
    pub fn new(buffers: &Vec<gltf::buffer::Data>, primitive: &gltf::Primitive) -> Self {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let indices = reader.read_indices().expect("No indices found!").into_u32().collect::<Vec<u32>>();
        let positions = reader.read_positions().expect("No positions found!").collect::<Vec<[f32; 3]>>();
        let normals = reader.read_normals().expect("No normals found!").collect::<Vec<[f32; 3]>>();
        let tangents = reader.read_tangents().expect("No tangents found!").collect::<Vec<[f32; 4]>>();
        let tex_coords = reader
            .read_tex_coords(0)
            .expect("No tex_coords found!")
            .into_f32()
            .collect::<Vec<[f32; 2]>>();
        let mut joints = vec![[0u32; 4]; positions.len()];
        let mut weights = vec![[0.0; 4]; positions.len()];

        let is_animated = reader.read_joints(0).is_some() && reader.read_weights(0).is_some();
        if is_animated {
            if let Some(read_joints) = reader.read_joints(0) {
                joints = read_joints
                    .into_u16()
                    .map(|j| [j[0] as u32, j[1] as u32, j[2] as u32, j[3] as u32]) // TODO: U16
                    .collect();
            }

            if let Some(read_weights) = reader.read_weights(0) {
                weights = read_weights.into_f32().collect();
            }
        }

        let material = primitive.material().index().expect("No material found!");
        let mut vertices = vec![];
        for i in 0..positions.len() {
            vertices.push(Vertex {
                position: positions[i],
                normal: normals[i],
                tangent: tangents[i],
                tex_coord: tex_coords[i],
                joints: joints[i],
                weights: weights[i],
            });
        }

        let length = indices.len() as u32;
        Self {
            indices,
            vertices,
            material,
            length,
        }
    }
}
