extern crate glium;

use vertex::Vertex;

use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use std::option::Option;
use std::collections::HashMap;

const BLOCK_SIZE: f32 = 0.5;
const CHUNK_SIZE: usize = 32;

pub fn make_cube<F: ? Sized>(facade: &F, x: f32, y: f32, z: f32) -> VertexBuffer<Vertex> where F: Facade {
    VertexBuffer::new(facade, &[
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),

        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),

        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]),

        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),

        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),

        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.0]),
    ]).unwrap()
}

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
        let _x = self.get_x(x) as f32;
        let _y = self.get_y(y) as f32;
        let _z = self.get_z(z) as f32;
        match self.blocks.get(&(x, y, z)) {
            Some(block_type) => match block_type {
                &BlockType::Empty => None,
                &BlockType::Solid => Some(make_cube(facade, _x, _y, _z)),
            }
            _ => None,
        }
    }
}

pub struct World {
    chunks: HashMap<(i32, i32, i32), Chunk>
}

impl World {
    pub fn new() -> World {
        World { chunks: HashMap::new() }
    }

    fn create_chunk(&mut self, x: i32, y: i32, z: i32) {
        let mut chunk = Chunk::new(0, 0, 0);
        for z in 0..32 {
            chunk.set((1, 1, z), BlockType::Solid);
            chunk.set((2, (z * 2 + 1) % 32, 10), BlockType::Solid);
        }
        match self.chunks.insert((x, y, z), chunk) {
            _ => ()
        }
    }

    pub fn get(&mut self, x: i32, y: i32, z: i32) -> &Chunk {
        if !self.chunks.contains_key(&(x, y, z)) {
            self.create_chunk(x, y, z)
        }
        match self.chunks.get(&(x, y, z)) {
            Some(chunk) => &chunk,
            None => panic!(),
        }
    }
}
