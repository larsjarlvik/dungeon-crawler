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

        let indices = reader.read_indices().unwrap().into_u32().collect::<Vec<u32>>();
        let positions = reader.read_positions().unwrap().collect::<Vec<[f32; 3]>>();
        let normals = reader.read_normals().unwrap().collect::<Vec<[f32; 3]>>();
        let tangents = reader.read_tangents().unwrap().collect::<Vec<[f32; 4]>>();
        let tex_coords = reader.read_tex_coords(0).unwrap().into_f32().collect::<Vec<[f32; 2]>>();
        let material = primitive.material().index().expect(&format!("No material found!"));

        let mut vertices = vec![];
        for i in 0..positions.len() {
            vertices.push(Vertex {
                position: positions[i],
                normal: normals[i],
                tangent: tangents[i],
                tex_coord: tex_coords[i],
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
