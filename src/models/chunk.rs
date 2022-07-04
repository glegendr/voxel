use crate::{
    models::cube::Cube,
    opengl::buffer::{buffer_data, Buffer, BufferType},
    opengl::mesh::{Mesh, Vertex},
};
use ogl33::*;

/// A group of `CHUNK_SIZE`³ cubes used to optimize OpenGl rendering
pub struct Chunk {
    cubes: Vec<Cube>,
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            cubes: vec![Cube::default(); Self::CHUNK_SIZE * Self::CHUNK_SIZE * Self::CHUNK_SIZE],
        }
    }
}

impl Chunk {
    /// size of chunk
    const CHUNK_SIZE: usize = 16;

    /// Create an empty Chunk
    pub fn empty() -> Self {
        Self {
            cubes: Vec::with_capacity(Self::CHUNK_SIZE * Self::CHUNK_SIZE * Self::CHUNK_SIZE),
        }
    }

    /// Render Chunk
    pub fn render(&self) -> Result<(), String> {
        let mut mesh: Mesh = Mesh::default();
        self.cubes.iter().enumerate().for_each(|(i, cube)| {
            if cube.is_active {
                create_cube_vertex(i, cube, &mut mesh)
            }
        });

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe { 
            glGenVertexArrays(1, &mut vao);
            glGenBuffers(1, &mut vbo);
            glGenBuffers(1, &mut ebo);
            glBindVertexArray(vao);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            glBufferData(GL_ARRAY_BUFFER, (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as isize, std::mem::transmute::<&Vec<Vertex>, *const _>(&mesh.vertices), GL_STATIC_DRAW);  
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

            glBufferData(GL_ELEMENT_ARRAY_BUFFER, (mesh.indices.len() * std::mem::size_of::<usize>()) as isize, 
            std::mem::transmute::<&Vec<usize>, *const _>(&mesh.indices), GL_STATIC_DRAW);

            // vertex positions
            glEnableVertexAttribArray(0);	
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, std::mem::size_of::<Vertex>() as i32, 0 as *const _);
            // vertex normals
            glEnableVertexAttribArray(1);	
            glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, std::mem::size_of::<Vertex>() as i32, std::mem::size_of::<[f32; 3]>() as *const _);
            // vertex texture coords
            glEnableVertexAttribArray(2);	
            glVertexAttribPointer(2, 2, GL_FLOAT, GL_FALSE, std::mem::size_of::<Vertex>() as i32, std::mem::size_of::<[f32; 6]>() as *const _);

            glBindVertexArray(0);
        };

        Ok(())
    }

    fn get_position(i: usize) -> (usize, usize, usize) {
        (
            i % Self::CHUNK_SIZE,
            (i / Self::CHUNK_SIZE) % Self::CHUNK_SIZE,
            i / (Self::CHUNK_SIZE * Self::CHUNK_SIZE),
        )
    }
}

pub fn create_cube_vertex(i: usize, _cube: &Cube, mesh: &mut Mesh) {
    let (x, y, z) = Chunk::get_position(i);
    let p_s = [
        [
            (x - Cube::CUBE_RENDER_SIZE) as f32,
            (y - Cube::CUBE_RENDER_SIZE) as f32,
            (z + Cube::CUBE_RENDER_SIZE) as f32,
        ], // p1
        [
            (x + Cube::CUBE_RENDER_SIZE) as f32,
            (y - Cube::CUBE_RENDER_SIZE) as f32,
            (z + Cube::CUBE_RENDER_SIZE) as f32,
        ], // p2
        [
            (x + Cube::CUBE_RENDER_SIZE) as f32,
            (y + Cube::CUBE_RENDER_SIZE) as f32,
            (z + Cube::CUBE_RENDER_SIZE) as f32,
        ], // p3
        [
            (x - Cube::CUBE_RENDER_SIZE) as f32,
            (y + Cube::CUBE_RENDER_SIZE) as f32,
            (z + Cube::CUBE_RENDER_SIZE) as f32,
        ], // p4
        [
            (x + Cube::CUBE_RENDER_SIZE) as f32,
            (y - Cube::CUBE_RENDER_SIZE) as f32,
            (z - Cube::CUBE_RENDER_SIZE) as f32,
        ], // p5
        [
            (x - Cube::CUBE_RENDER_SIZE) as f32,
            (y - Cube::CUBE_RENDER_SIZE) as f32,
            (z - Cube::CUBE_RENDER_SIZE) as f32,
        ], // p6
        [
            (x - Cube::CUBE_RENDER_SIZE) as f32,
            (y + Cube::CUBE_RENDER_SIZE) as f32,
            (z - Cube::CUBE_RENDER_SIZE) as f32,
        ], // p7
        [
            (x + Cube::CUBE_RENDER_SIZE) as f32,
            (y + Cube::CUBE_RENDER_SIZE) as f32,
            (z - Cube::CUBE_RENDER_SIZE) as f32,
        ], // p8
    ];
    let color: [f32; 4] = [1., 1., 1., 1.];

    // Front
    let mut normal: [f32; 3] = [0., 0., 1.];
    let mut v1 = mesh.push_vertex(Vertex::new(p_s[0], normal, color));
    let mut v2 = mesh.push_vertex(Vertex::new(p_s[1], normal, color));
    let mut v3 = mesh.push_vertex(Vertex::new(p_s[2], normal, color));
    let mut v4 = mesh.push_vertex(Vertex::new(p_s[3], normal, color));
    mesh.push_triangle(v1, v2, v3);
    mesh.push_triangle(v1, v3, v4);

    // Back
    normal[2] = -1.;
    let mut v5 = mesh.push_vertex(Vertex::new(p_s[4], normal, color));
    let mut v6 = mesh.push_vertex(Vertex::new(p_s[5], normal, color));
    let mut v7 = mesh.push_vertex(Vertex::new(p_s[6], normal, color));
    let mut v8 = mesh.push_vertex(Vertex::new(p_s[7], normal, color));
    mesh.push_triangle(v5, v6, v7);
    mesh.push_triangle(v5, v7, v8);

    // Right
    normal = [1., 0., 0.];
    v2 = mesh.push_vertex(Vertex::new(p_s[1], normal, color));
    v5 = mesh.push_vertex(Vertex::new(p_s[4], normal, color));
    v8 = mesh.push_vertex(Vertex::new(p_s[7], normal, color));
    v3 = mesh.push_vertex(Vertex::new(p_s[2], normal, color));
    mesh.push_triangle(v2, v5, v8);
    mesh.push_triangle(v2, v8, v3);

    // Left
    normal[0] = -1.;
    v6 = mesh.push_vertex(Vertex::new(p_s[5], normal, color));
    v1 = mesh.push_vertex(Vertex::new(p_s[0], normal, color));
    v4 = mesh.push_vertex(Vertex::new(p_s[3], normal, color));
    v7 = mesh.push_vertex(Vertex::new(p_s[6], normal, color));
    mesh.push_triangle(v6, v1, v4);
    mesh.push_triangle(v6, v4, v7);

    // Top
    normal = [0., 1., 0.];
    v4 = mesh.push_vertex(Vertex::new(p_s[3], normal, color));
    v3 = mesh.push_vertex(Vertex::new(p_s[2], normal, color));
    v8 = mesh.push_vertex(Vertex::new(p_s[7], normal, color));
    v7 = mesh.push_vertex(Vertex::new(p_s[6], normal, color));
    mesh.push_triangle(v4, v3, v8);
    mesh.push_triangle(v4, v8, v7);

    // Bottom
    normal[1] = -1.;
    v6 = mesh.push_vertex(Vertex::new(p_s[5], normal, color));
    v5 = mesh.push_vertex(Vertex::new(p_s[4], normal, color));
    v2 = mesh.push_vertex(Vertex::new(p_s[1], normal, color));
    v1 = mesh.push_vertex(Vertex::new(p_s[0], normal, color));
    mesh.push_triangle(v6, v5, v2);
    mesh.push_triangle(v6, v2, v1);
}
