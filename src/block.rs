extern crate glium;

use vertex::Vertex;

use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use std::option::Option;
use std::collections::HashMap;

const BLOCK_SIZE: f32 = 0.5;
const CHUNK_SIZE: usize = 32;

pub enum BlockType {
    Empty,
    Solid,
}

pub type ChunkPosition = (u8, u8, u8);

pub struct Chunk {
    x: i32,
    y: i32,
    z: i32,
    blocks: HashMap<ChunkPosition, BlockType>,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Chunk {
        Chunk { x, y, z, blocks: HashMap::new() }
    }

    pub fn set(&mut self, position: ChunkPosition, block_type: BlockType) {
        self.blocks.insert(position, block_type);
    }

    pub fn get_x(&self, x: u8) -> i32 {
        (self.x * CHUNK_SIZE as i32) + x as i32
    }

    pub fn get_y(&self, y: u8) -> i32 {
        (self.y * CHUNK_SIZE as i32) + y as i32
    }

    pub fn get_z(&self, z: u8) -> i32 {
        (self.z * CHUNK_SIZE as i32) + z as i32
    }

    pub fn get_vertices<F: ? Sized>(&self, facade: &F, x: u8, y: u8, z: u8) -> Option<VertexBuffer<Vertex>> where F: Facade {
        match self.blocks.get(&(x, y, z)) {
            Some(block_type) => match block_type {
                &BlockType::Empty => None,
                &BlockType::Solid => Some(VertexBuffer::new(facade, &[
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
            }
            _ => None,
        }
    }
}
