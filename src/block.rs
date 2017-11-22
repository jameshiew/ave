extern crate glium;

use vertex::Vertex;
use space::Position;
use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use std::collections::HashMap;

const BLOCK_SIZE: f32 = 0.5;
const CHUNK_SIZE: usize = 32;

pub fn make_cube<F: ? Sized>(facade: &F, x: f32, y: f32, z: f32) -> VertexBuffer<Vertex> where F: Facade {
    VertexBuffer::new(facade, &[
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),

        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.3, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.3, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.3, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [-1.0, 0.0, 0.0], [0.3, 1.0, 0.0]),

        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, -1.0, 0.0], [0.2, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, -1.0, 0.0], [0.2, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, -1.0, 0.0], [0.2, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, -1.0, 0.0], [0.2, 0.0, 1.0]),

        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]),

        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),

        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.4]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.4]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.4]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.0, 0.0, -1.0], [1.0, 1.0, 0.4]),
    ]).unwrap()
}

pub enum BlockType {
    Empty,
    Solid,
}

pub type ChunkPosition = (u8, u8, u8);

pub struct Chunk {
    pub blocks: HashMap<ChunkPosition, BlockType>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {blocks: HashMap::new() }
    }

    pub fn set(&mut self, position: ChunkPosition, block_type: BlockType) {
        self.blocks.insert(position, block_type);
    }

    pub fn get(&mut self, position: ChunkPosition) -> &BlockType {
        match self.blocks.get(&position) {
            Some(block_type) => block_type,
            None => &BlockType::Empty,
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

    fn create_chunk(&mut self, wx: i32, wy: i32, wz: i32) {
        let mut chunk = Chunk::new();
        if wy < 0 {  // underground
            for x in 0..32 {
                for y in 0..32 {
                    for z in 0..32 {
                        chunk.set((x, y, z), BlockType::Solid);
                    }
                }
            }
        } else if wy == 0 {
            for y in 0..12 {
                chunk.set((7, y, 7), BlockType::Solid);
            }
        }
        match self.chunks.insert((wx, wy, wz), chunk) {
            _ => ()
        }
    }

    pub fn get(&mut self, wx: i32, wy: i32, wz: i32) -> &Chunk {
        if !self.chunks.contains_key(&(wx, wy, wz)) {
            self.create_chunk(wx, wy, wz)
        }
        match self.chunks.get(&(wx, wy, wz)) {
            Some(chunk) => &chunk,
            None => panic!(),
        }
    }

    pub fn get_position(wx: i32, wy: i32, wz: i32, cx: u8, cy: u8, cz: u8) -> Position {
        let x = (wx * CHUNK_SIZE as i32) + cx as i32;
        let y = (wy * CHUNK_SIZE as i32) + cy as i32;
        let z = (wz * CHUNK_SIZE as i32) + cz as i32;
        return Position(x as f32, y as f32, z as f32);
    }
}
