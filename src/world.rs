use space::Position;
use std::collections::{HashMap, HashSet};
use block::{BLOCKS, BlockType};
use cgmath::Vector3;
use worldgen::{FlatWorldGenerator, WorldGenerator};
use camera::CameraState;
use std::vec::Vec;
use worldgen;

/// Side length of a chunk (in blocks) - all chunks are cubic
pub const CHUNK_SIZE: u8 = 32;

/// Indicates an index into a chunk with dimensions CHUNK_SIZE x CHUNK_SIZE x CHUNK_SIZE
pub type BlockCoordinates = Vector3<u8>;

pub struct Chunk {
    /// Each chunk position is mapped to an index into the BLOCKS slice
    ///
    /// Absence of a chunk position key indicated an empty (air) block
    pub blocks: HashMap<BlockCoordinates, usize>,
    /// Chunk positions which are completely occluded and so should never be rendered
    pub mask: HashSet<BlockCoordinates>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { blocks: HashMap::new(), mask: HashSet::new() }
    }

    /// get adjacent positions - ignoring diagonals
    pub fn get_adjacent(position: BlockCoordinates) -> HashSet<BlockCoordinates> {
        let mut set: HashSet<BlockCoordinates> = HashSet::new();
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

    pub fn set(&mut self, position: BlockCoordinates, block_type: usize) {
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

    pub fn remove(&mut self, position: BlockCoordinates) {
        self.blocks.remove(&position);
        for adjacent_position in Chunk::get_adjacent(position) {
            if !self.is_occluded(adjacent_position) {
                self.mask.remove(&adjacent_position);
            }
        }
    }

    pub fn get(&self, position: BlockCoordinates) -> Option<&BlockType> {
        match self.blocks.get(&position) {
            Some(block_type) => Some(BLOCKS[*block_type]),
            None => None,
        }
    }

    pub fn is_occluded(&self, position: BlockCoordinates) -> bool {
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
    pub fn get_visible(&self) -> HashSet<(BlockCoordinates, &BlockType)> {
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

pub type ChunkCoordinates = Vector3<i32>;

pub fn get_position(chunk_coordinates: &ChunkCoordinates, block_coordinates: &BlockCoordinates) -> Position {
    let x = (chunk_coordinates[0] * CHUNK_SIZE as i32) + block_coordinates[0] as i32;
    let y = (chunk_coordinates[1] * CHUNK_SIZE as i32) + block_coordinates[1] as i32;
    let z = (chunk_coordinates[2] * CHUNK_SIZE as i32) + block_coordinates[2] as i32;
    return [x as f32, y as f32, z as f32].into();
}

pub fn position_to_chunk(coordinates: &Position) -> ChunkCoordinates {
    ((coordinates[0] / CHUNK_SIZE as f32) as i32, (coordinates[1] / CHUNK_SIZE as f32) as i32, (coordinates[2] / CHUNK_SIZE as f32) as i32).into()
}

pub trait World {
    fn new() -> Self;
    fn get_or_create(&mut self, coordinates: ChunkCoordinates) -> &Chunk;
    fn get_visible(&mut self, camera: &CameraState) -> Vec<(Position, &BlockType)>;
}

pub struct InMemoryWorld {
    generator: Box<WorldGenerator>,
    chunks: HashMap<ChunkCoordinates, Chunk>,
}

impl World for InMemoryWorld {
    fn new() -> InMemoryWorld {
        InMemoryWorld { generator: Box::new(worldgen::RandomPillarsWorldGenerator::new(192)), chunks: HashMap::new() }
    }

    fn get_or_create(&mut self, coordinates: ChunkCoordinates) -> &Chunk {
        if self.chunks.contains_key(&coordinates) {
            return self.chunks.get(&coordinates).unwrap();
        } else {
            let chunk = self.generator.generate_chunk(coordinates);
            self.chunks.insert(coordinates, chunk);
            return self.chunks.get_mut(&coordinates).unwrap()
        }
    }

    fn get_visible(&mut self, camera: &CameraState) -> Vec<(Position, &BlockType)> {
        // for now, just return blocks of current chunk
        let mut vec = Vec::new();
        let chunk_coordinates = position_to_chunk(&camera.position);
        let chunk = self.get_or_create(chunk_coordinates);
        for (block_coordinates, block_type) in chunk.get_visible() {
            vec.push((get_position(&chunk_coordinates, &block_coordinates), block_type))
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use space::Position;
    use block::{BlockType, BLOCKS};
    use std::collections::{HashMap, HashSet};
    use world::{InMemoryWorld, get_position, position_to_chunk, Chunk, CHUNK_SIZE};

    #[test]
    fn world_get_position() {
        assert_eq!(get_position([0, 0, 0].into(), [0, 0, 0].into()), [0.0, 0.0, 0.0].into());
        assert_eq!(get_position([0, 0, 0].into(), [1, 1, 1].into()), [1.0, 1.0, 1.0].into());
        assert_eq!(get_position([1, 1, 1].into(), [1, 1, 1].into()), [CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0].into());
    }

    #[test]
    fn world_get_chunk_xyz() {
        assert_eq!(position_to_chunk([0.0, 0.0, 0.0].into()), [0, 0, 0].into());
        assert_eq!(position_to_chunk([10.0, 12.0, 15.0].into()), [0, 0, 0].into());
    }

    #[test]
    fn chunk_get() {
        let mut chunk = Chunk::new();
        chunk.set([0, 0, 0].into(), 1);
        assert_eq!(chunk.get([0, 0, 0].into()), Some(BLOCKS[1]));
    }

    #[test]
    fn chunk_get_adjacent() {
        let mut origin_adjacent = HashSet::new();
        origin_adjacent.insert([1u8, 0u8, 0u8].into());
        origin_adjacent.insert([0u8, 1u8, 0u8].into());
        origin_adjacent.insert([0u8, 0u8, 1u8].into());
        assert_eq!(Chunk::get_adjacent([0, 0, 0].into()), origin_adjacent);

        let mut general_adjacent = HashSet::new();
        general_adjacent.insert([6u8, 5u8, 5u8].into());
        general_adjacent.insert([5u8, 6u8, 5u8].into());
        general_adjacent.insert([5u8, 5u8, 6u8].into());
        general_adjacent.insert([4u8, 5u8, 5u8].into());
        general_adjacent.insert([5u8, 4u8, 5u8].into());
        general_adjacent.insert([5u8, 5u8, 4u8].into());
        assert_eq!(Chunk::get_adjacent([5, 5, 5].into()), general_adjacent);
    }

    #[test]
    fn chunk_is_occluded() {
        let mut chunk = Chunk::new();
        chunk.set([3, 3, 3].into(), 1);
        chunk.set([3, 3, 2].into(), 1);
        chunk.set([5, 5, 5].into(), 1);
        assert!(!chunk.is_occluded([3, 3, 3].into()));
        chunk.set([3, 2, 3].into(), 1);
        chunk.set([2, 3, 3].into(), 1);
        chunk.set([4, 3, 3].into(), 1);
        chunk.set([3, 4, 3].into(), 1);
        chunk.set([3, 3, 4].into(), 1);
        assert!(chunk.is_occluded([3, 3, 3].into()));
    }
}
