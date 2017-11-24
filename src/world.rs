extern crate glium;

use space::Position;
use std::collections::{HashMap, HashSet};
use block::{BLOCKS, BlockType};
use cgmath::Vector3;

/// Side length of a chunk (in blocks) - all chunks are cubic
const CHUNK_SIZE: u8 = 32;

/// Indicates an index into a chunk with dimensions CHUNK_SIZE x CHUNK_SIZE x CHUNK_SIZE
pub type ChunkPosition = Vector3<u8>;

pub struct Chunk {
    /// Each chunk position is mapped to an index into the BLOCKS slice
    ///
    /// Absence of a chunk position key indicated an empty (air) block
    pub blocks: HashMap<ChunkPosition, usize>,
    /// Chunk positions which are completely occluded and so should never be rendered
    pub mask: HashSet<ChunkPosition>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { blocks: HashMap::new(), mask: HashSet::new() }
    }

    /// get adjacent positions - ignoring diagonals
    pub fn get_adjacent(position: ChunkPosition) -> HashSet<ChunkPosition> {
        let mut set: HashSet<ChunkPosition> = HashSet::new();
        if position[0] < CHUNK_SIZE - 1 {
            set.insert([position[0] + 1u8, position[1], position[2]].into());
        }
        if position[1] < CHUNK_SIZE - 1 {
            set.insert([position[0], position[1] + 1u8, position[2]].into());
        }
        if position[2] < CHUNK_SIZE - 1 {
            set.insert([position[0], position[1], position[2] + 1u8].into());
        }
        if position[0] > 0 {
            set.insert([position[0] - 1u8, position[1], position[2]].into());
        }
        if position[1] > 0 {
            set.insert([position[0], position[1] - 1u8, position[2]].into());
        }
        if position[2] > 0 {
            set.insert([position[0], position[1], position[2] - 1u8].into());
        }
        return set;
    }

    pub fn set(&mut self, position: ChunkPosition, block_type: usize) {
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

    pub fn remove(&mut self, position: ChunkPosition) {
        self.blocks.remove(&position);
        for adjacent_position in Chunk::get_adjacent(position) {
            if !self.is_occluded(adjacent_position) {
                self.mask.remove(&adjacent_position);
            }
        }
    }

    pub fn get(&self, position: ChunkPosition) -> Option<&BlockType> {
        match self.blocks.get(&position) {
            Some(block_type) => Some(BLOCKS[*block_type]),
            None => None,
        }
    }

    pub fn is_occluded(&self, position: ChunkPosition) -> bool {
        if [0, CHUNK_SIZE - 1].contains(&position[0]) || [0, CHUNK_SIZE - 1].contains(&position[1]) || [0, CHUNK_SIZE - 1].contains(&position[2]) {
            return false;  // cheating by for now always showing blocks that are on the edge of chunks
        }
        for adjacent_position in Chunk::get_adjacent(position) {
            match self.get(adjacent_position) {
                None => return false,
                Some(_) => ()
            }
        }
        return true;
    }

    /// ideally this would be a lazy iterator - but need to think about lifetimes etc
    pub fn get_visible(&self) -> HashSet<(ChunkPosition, &BlockType)> {
        let mut visible = HashSet::new();
        for (chunk_position, block_type) in self.blocks.iter() {
            match self.mask.get(chunk_position) {
                Some(_) => continue,
                None => visible.insert((*chunk_position, BLOCKS[*block_type])),
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
                        chunk.set([x, y, z].into(), 1);
                    }
                }
            }
        } else if wy == 0 {
            for y in 0..12 {
                chunk.set([7, y, 7].into(), 0);
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

    pub fn get_chunk_xyz(x: f32, y: f32, z: f32) -> (i32, i32, i32) {
        ((x / CHUNK_SIZE as f32) as i32, (y / CHUNK_SIZE as f32) as i32, (z / CHUNK_SIZE as f32) as i32)
    }
}

#[cfg(test)]
mod tests {
    use space::Position;
    use block::{BlockType, BLOCKS};
    use std::collections::{HashMap, HashSet};
    use world::{World, Chunk, CHUNK_SIZE};

    #[test]
    fn world_get_position() {
        assert_eq!(World::get_position(0, 0, 0, 0, 0, 0), Position(0.0, 0.0, 0.0));
        assert_eq!(World::get_position(0, 0, 0, 1, 1, 1), Position(1.0, 1.0, 1.0));
        assert_eq!(World::get_position(1, 1, 1, 1, 1, 1), Position(CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0));
    }

    #[test]
    fn world_get_chunk_xyz() {
        assert_eq!(World::get_chunk_xyz(0.0, 0.0, 0.0), (0, 0, 0));
        assert_eq!(World::get_chunk_xyz(10.0, 12.0, 15.0), (0, 0, 0));
    }

    #[test]
    fn chunk_get() {
        let mut chunk = Chunk::new();
        chunk.set((0, 0, 0), 1);
        assert_eq!(chunk.get((0, 0, 0)), Some(BLOCKS[1]));
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
        chunk.set((3, 3, 3), 1);
        chunk.set((3, 3, 2), 1);
        chunk.set((5, 5, 5), 1);
        assert!(!chunk.is_occluded((3, 3, 3)));
        chunk.set((3, 2, 3), 1);
        chunk.set((2, 3, 3), 1);
        chunk.set((4, 3, 3), 1);
        chunk.set((3, 4, 3), 1);
        chunk.set((3, 3, 4), 1);
        assert!(chunk.is_occluded((3, 3, 3)));
    }
}
