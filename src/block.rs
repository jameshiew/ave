extern crate glium;

use vertex::Vertex;

use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use std::option::Option;

const BLOCK_SIZE: f32 = 0.5;
const CHUNK_SIZE: usize = 32;

#[derive(Copy, Clone)]
pub struct Chunk {
    x: i32,
    y: i32,
    z: i32,
    blocks: [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Chunk {
        Chunk { x, y, z, blocks: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block_id: u32) {
        self.blocks[x][y][z] = block_id;
    }

    pub fn get_x(&self, x: usize) -> i32 {
        self.x * (CHUNK_SIZE + x) as i32
    }

    pub fn get_y(&self, y: usize) -> i32 {
        self.y * (CHUNK_SIZE + y) as i32
    }

    pub fn get_z(&self, z: usize) -> i32 {
        self.z * (CHUNK_SIZE + z) as i32
    }

    pub fn get_vertices<F: ? Sized>(&self, facade: &F, x: usize, y: usize, z: usize) -> Option<VertexBuffer<Vertex>> where F: Facade {
        match self.blocks[x][y][z] {
            1 => Some(VertexBuffer::new(facade, &[
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},

                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},

                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0]},

                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},

                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 - BLOCK_SIZE], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0]},

                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 - BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 - BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0]},
                Vertex { position: [self.get_x(x) as f32 + BLOCK_SIZE, self.get_y(y) as f32 + BLOCK_SIZE, self.get_z(z) as f32 + BLOCK_SIZE], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0]}
            ]).unwrap()),
            _ => None,
        }
    }
}
