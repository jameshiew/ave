use space::Position;
use std::collections::{HashMap, HashSet};
use block::{BLOCKS, BlockType};
use cgmath::Point3;
use worldgen::WorldGenerator;
use std::vec::Vec;
use space::Adjacent;
use worldgen;
use block;

/// Side length of a chunk (in blocks) - all chunks are cubic
pub const CHUNK_SIZE: u8 = 32;

/// Indicates an index into a chunk with dimensions CHUNK_SIZE x CHUNK_SIZE x CHUNK_SIZE
pub type BlockCoordinates = Point3<u8>;

pub struct Chunk {
    /// Each chunk position is mapped to an index into the BLOCKS slice
    ///
    /// Absence of a chunk position key indicated an empty (air) block
    pub blocks: HashMap<BlockCoordinates, block::ID>,
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

    pub fn set(&mut self, position: BlockCoordinates, block_type: block::ID) {
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

pub type ChunkCoordinates = Point3<i32>;

impl Adjacent for Point3<i32> {
    fn adjacent(&self) -> Vec<Self> {
        let mut vec = self.directly_adjacent();
        vec.append(&mut self.diagonally_adjacent());
        vec
    }

    fn directly_adjacent(&self) -> Vec<Self> {
        let mut vec = Vec::new();
        vec.push([self[0] + 1, self[1], self[2]].into());
        vec.push([self[0], self[1] + 1, self[2]].into());
        vec.push([self[0], self[1], self[2] + 1].into());
        vec.push([self[0] - 1, self[1], self[2]].into());
        vec.push([self[0], self[1] - 1, self[2]].into());
        vec.push([self[0], self[1], self[2] - 1].into());
        vec
    }

    fn diagonally_adjacent(&self) -> Vec<Self> {
        let mut vec = Vec::new();
        vec.push([self[0] + 1, self[1] + 1, self[2] + 1].into());
        vec.push([self[0] - 1, self[1] + 1, self[2] + 1].into());
        vec.push([self[0] - 1, self[1] - 1, self[2] + 1].into());
        vec.push([self[0] - 1, self[1] - 1, self[2] - 1].into());
        vec.push([self[0] - 1, self[1] + 1, self[2] - 1].into());
        vec.push([self[0] + 1, self[1] + 1, self[2] - 1].into());
        vec.push([self[0] + 1, self[1] - 1, self[2] - 1].into());
        vec.push([self[0] + 1, self[1] - 1, self[2] + 1].into());
        vec
    }
}

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
    fn at(&self, position: Position, radius: u8) -> Vec<(Position, &BlockType)>;
}

pub struct InMemoryWorld {
    generator: Box<WorldGenerator>,
    chunks: HashMap<ChunkCoordinates, Chunk>,
}

impl World for InMemoryWorld {
    fn new() -> InMemoryWorld {
        InMemoryWorld { generator: Box::new(worldgen::NaturalWorldGenerator::new()), chunks: HashMap::new() }
    }

    fn get_or_create(&mut self, coordinates: ChunkCoordinates) -> &Chunk {
        if self.chunks.contains_key(&coordinates) {
            return self.chunks.get(&coordinates).unwrap();
        } else {
            println!("Generating chunk at {}, {}, {}", coordinates.x, coordinates.y, coordinates.z);
            let chunk = self.generator.generate_chunk(coordinates);
            self.chunks.insert(coordinates, chunk);
            return self.chunks.get_mut(&coordinates).unwrap();
        }
    }

    fn at(&self, position: Position, radius: u8) -> Vec<(Position, &BlockType)> {
        // for now, just return blocks of current nearby chunks
        let mut chunk_coordinates_to_render = HashSet::new();
        let current_chunk_coordinates = position_to_chunk(&position);
        chunk_coordinates_to_render.insert(current_chunk_coordinates);
        let iradius = radius as i32;
        for x in -iradius..iradius + 1 {
            for y in -iradius..iradius + 1 {
                for z in -iradius..iradius + 1 {
                    chunk_coordinates_to_render.insert(
                        [current_chunk_coordinates[0] + x,
                        current_chunk_coordinates[1] + y,
                        current_chunk_coordinates[2] + z].into()
                    );
                }
            }
        }

        let mut blocks = Vec::new();
        for chunk_coordinates in chunk_coordinates_to_render {
            let chunk_opt = self.chunks.get(&chunk_coordinates);
            match chunk_opt {
                Some(chunk) => {
                    for (block_coordinates, block_type) in chunk.get_visible() {
                        blocks.push((get_position(&chunk_coordinates, &block_coordinates), block_type))
                    }
                },
                None => (),
            }
        }
        blocks
    }
}

#[cfg(test)]
mod tests {
    use block::BLOCKS;
    use std::collections::HashSet;
    use world::{get_position, position_to_chunk, Chunk, CHUNK_SIZE};

    #[test]
    fn world_get_position() {
        assert_eq!(get_position(&[0, 0, 0].into(), &[0, 0, 0].into()), [0.0, 0.0, 0.0].into());
        assert_eq!(get_position(&[0, 0, 0].into(), &[1, 1, 1].into()), [1.0, 1.0, 1.0].into());
        assert_eq!(get_position(&[1, 1, 1].into(), &[1, 1, 1].into()), [CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0, CHUNK_SIZE as f32 + 1.0].into());
    }

    #[test]
    fn world_get_chunk_xyz() {
        assert_eq!(position_to_chunk(&[0.0, 0.0, 0.0].into()), [0, 0, 0].into());
        assert_eq!(position_to_chunk(&[10.0, 12.0, 15.0].into()), [0, 0, 0].into());
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
