/* Vertex */
/// definition of a point in space, his normal and his texture
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 4]) -> Self {
        Vertex {
            position,
            normal,
            color,
        }
    }
}
/* Texture */
// struct Texture {
//     id: u32,
//     type_: String
// }
/* Mesh */
/// Single drawable entity
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
    // texture: Vec<Texture>
}

impl Mesh {
    /// create a new structure Mesh
    pub fn new(vertices: Vec<Vertex>, indices: Vec<usize>) -> Self {
        Mesh { vertices, indices }
    }

    /// push a vertex at the end of the Mesh
    pub fn push_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    /// push a triangle at the end of the Mesh
    pub fn push_triangle(&mut self, v1: usize, v2: usize, v3: usize) {
        self.indices.push(v1);
        self.indices.push(v2);
        self.indices.push(v3);
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            vertices: Vec::default(),
            indices: Vec::default(),
        }
    }
}
