extern crate glium;

use vertex::Vertex;
use space::Position;
use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use std::collections::{HashMap, HashSet};

const BLOCK_SIZE: f32 = 0.5;
const CHUNK_SIZE: u8 = 32;

pub fn make_cube<F: ? Sized>(facade: &F, x: f32, y: f32, z: f32) -> VertexBuffer<Vertex> where F: Facade {
    VertexBuffer::new(facade, &[
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.3, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [0.3, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.3, 1.0, 0.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.3, 1.0, 0.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.2, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.2, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.2, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [0.2, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [1.0, 0.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 0.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 1.0]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z - BLOCK_SIZE], [0.0, 1.0, 1.0]),
        Vertex::new([x - BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 1.0, 0.4]),
        Vertex::new([x - BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 1.0, 0.4]),
        Vertex::new([x + BLOCK_SIZE, y - BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 1.0, 0.4]),
        Vertex::new([x + BLOCK_SIZE, y + BLOCK_SIZE, z + BLOCK_SIZE], [1.0, 1.0, 0.4]),
    ]).unwrap()
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum BlockType {
    Empty,
    Solid,
}

pub type ChunkPosition = (u8, u8, u8);

pub struct Chunk {
    pub blocks: HashMap<ChunkPosition, BlockType>,
    /// chunk positions which are completely occluded and so should never be rendered
    pub mask: HashSet<ChunkPosition>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { blocks: HashMap::new(), mask: HashSet::new() }
    }

    pub fn set(&mut self, position: ChunkPosition, block_type: BlockType) {
        match block_type {
            BlockType::Empty => {
                self.blocks.remove(&position);
                for adjacent_position in Chunk::get_adjacent(position) {
                    if !self.is_occluded(adjacent_position) {
                        self.mask.remove(&adjacent_position);
                    }
                }
            },
            _ => {
                self.blocks.insert(position, block_type);
                if self.is_occluded(position) {
                    self.mask.insert(position);
                }
                for adjacent_position in Chunk::get_adjacent(position) {
                    if self.is_occluded(adjacent_position) {
                        self.mask.insert(adjacent_position);
                    }
                }
            }
        }
    }

    pub fn get(&self, position: ChunkPosition) -> &BlockType {
        match self.blocks.get(&position) {
            Some(block_type) => block_type,
            None => &BlockType::Empty,
        }
    }

    /// get adjacent positions - ignoring diagonals
    pub fn get_adjacent(position: ChunkPosition) -> HashSet<ChunkPosition> {
        let mut set: HashSet<ChunkPosition> = HashSet::new();
        if position.0 < CHUNK_SIZE - 1 {
            set.insert((position.0 + 1u8, position.1, position.2));
        }
        if position.1 < CHUNK_SIZE - 1 {
            set.insert((position.0, position.1 + 1u8, position.2));
        }
        if position.2 < CHUNK_SIZE - 1 {
            set.insert((position.0, position.1, position.2 + 1u8));
        }
        if position.0 > 0 {
            set.insert((position.0 - 1u8, position.1, position.2));
        }
        if position.1 > 0 {
            set.insert((position.0, position.1 - 1u8, position.2));
        }
        if position.2 > 0 {
            set.insert((position.0, position.1, position.2 - 1u8));
        }
        return set;
    }

    pub fn is_occluded(&self, position: ChunkPosition) -> bool {
        if [0, CHUNK_SIZE - 1].contains(&position.0) || [0, CHUNK_SIZE - 1].contains(&position.1) || [0, CHUNK_SIZE - 1].contains(&position.2) {
            return false;  // cheating by for now always showing blocks that are on the edge of chunks
        }
        for adjacent_position in Chunk::get_adjacent(position) {
            match self.get(adjacent_position) {
                &BlockType::Empty => return false,
                _ => ()
            }
        }
        return true
    }

    /// ideally this would be a lazy iterator - but need to think about lifetimes etc
    pub fn get_visible(&self) -> HashSet<(ChunkPosition, &BlockType)> {
        let mut visible = HashSet::new();
        for (chunk_position, block_type) in self.blocks.iter() {
            match self.mask.get(chunk_position) {
                Some(_) => continue,
                None => visible.insert((*chunk_position, block_type)),
            };
        }
        return visible;
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
        if wy < 0 {
            // underground
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

    pub fn get_or_create(&mut self, wx: i32, wy: i32, wz: i32) -> &Chunk {
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

#[cfg(test)]
mod tests {
    use space::Position;
    use block::{World, Chunk, ChunkPosition, CHUNK_SIZE, BlockType};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn world_get_position() {
        assert_eq!(World::get_position(0, 0, 0, 0, 0, 0), Position(0.0, 0.0, 0.0));
        assert_eq!(World::get_position(0, 0, 0, 1, 1, 1), Position(1.0, 1.0, 1.0));
        assert_eq!(World::get_position(1, 1, 1, 1, 1, 1), Position(CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0));
    }

    #[test]
    fn chunk_get() {
        let mut chunk = Chunk::new();
        chunk.set((0, 0, 0), BlockType::Solid);
        assert_eq!(chunk.get((0, 0, 0)), &BlockType::Solid);
    }

    #[test]
    fn chunk_get_adjacent() {
        let mut origin_adjacent = HashSet::new();
        origin_adjacent.insert((1u8, 0u8, 0u8));
        origin_adjacent.insert((0u8, 1u8, 0u8));
        origin_adjacent.insert((0u8, 0u8, 1u8));
        assert_eq!(Chunk::get_adjacent((0, 0, 0)), origin_adjacent);

        let mut general_adjacent = HashSet::new();
        general_adjacent.insert((6u8, 5u8, 5u8));
        general_adjacent.insert((5u8, 6u8, 5u8));
        general_adjacent.insert((5u8, 5u8, 6u8));
        general_adjacent.insert((4u8, 5u8, 5u8));
        general_adjacent.insert((5u8, 4u8, 5u8));
        general_adjacent.insert((5u8, 5u8, 4u8));
        assert_eq!(Chunk::get_adjacent((5, 5, 5)), general_adjacent);
    }

    #[test]
    fn chunk_is_occluded() {
        let mut chunk = Chunk::new();
        chunk.set((3, 3, 3), BlockType::Solid);
        chunk.set((3, 3, 2), BlockType::Solid);
        chunk.set((5, 5, 5), BlockType::Solid);
        assert!(!chunk.is_occluded((3, 3, 3)));
        chunk.set((3, 2, 3), BlockType::Solid);
        chunk.set((2, 3, 3), BlockType::Solid);
        chunk.set((4, 3, 3), BlockType::Solid);
        chunk.set((3, 4, 3), BlockType::Solid);
        chunk.set((3, 3, 4), BlockType::Solid);
        assert!(chunk.is_occluded((3, 3, 3)));
    }
}
