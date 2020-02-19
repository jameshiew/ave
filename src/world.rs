use space::Position;
use std::collections::{HashMap, HashSet};
use block::BlockType;
use cgmath::Point3;
use worldgen::WorldGenerator;
use std::vec::Vec;
use space::Adjacent;
use worldgen;
use rand;

/// Side length of a chunk (in blocks) - all chunks are cubic
pub const CHUNK_SIZE: u8 = 32;

/// Indicates an index into a chunk with dimensions CHUNK_SIZE x CHUNK_SIZE x CHUNK_SIZE
pub type BlockCoordinates = Point3<u8>;

pub struct HashChunk {
    /// Each chunk position is mapped to an index into the BLOCKS slice
    ///
    /// Absence of a chunk position key indicated an empty (air) block
    pub blocks: HashMap<BlockCoordinates, &'static BlockType>,
    /// Chunk positions which are completely occluded and so should never be rendered
    pub mask: HashSet<BlockCoordinates>,
}

pub trait Chunk {
    /// get adjacent positions - ignoring diagonals
    fn get_adjacent(position: BlockCoordinates) -> HashSet<BlockCoordinates>;
    fn set(&mut self, position: BlockCoordinates, block_type: &'static BlockType);
    fn get(&self, position: BlockCoordinates) -> Option<&&'static BlockType>;
    fn is_occluded(&self, position: BlockCoordinates) -> bool;
    /// ideally this would be a lazy iterator - but need to think about lifetimes etc
    fn get_visible(&self) -> HashSet<(BlockCoordinates, &BlockType)>;
}

impl HashChunk {
    pub fn new() -> HashChunk {
        HashChunk { blocks: HashMap::new(), mask: HashSet::new() }
    }
}

impl Chunk for HashChunk {
    /// get adjacent positions - ignoring diagonals
    fn get_adjacent(position: BlockCoordinates) -> HashSet<BlockCoordinates> {
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

    fn set(&mut self, position: BlockCoordinates, block_type: &'static BlockType) {
        self.blocks.insert(position, block_type);
        if self.is_occluded(position) {
            self.mask.insert(position);
        }
        for adjacent_position in HashChunk::get_adjacent(position) {
            if self.is_occluded(adjacent_position) {
                self.mask.insert(adjacent_position);
            }
        }
    }

    fn get(&self, position: BlockCoordinates) -> Option<&&'static BlockType> {
        return self.blocks.get(&position);
    }

    fn is_occluded(&self, position: BlockCoordinates) -> bool {
        if [0, CHUNK_SIZE - 1].contains(&position[0]) || [0, CHUNK_SIZE - 1].contains(&position[1]) || [0, CHUNK_SIZE - 1].contains(&position[2]) {
            return false;  // cheating by for now always showing blocks that are on the edge of chunks
        }
        for adjacent_position in HashChunk::get_adjacent(position) {
            match self.get(adjacent_position) {
                None => return false,
                Some(_) => ()
            }
        }
        return true;
    }

    /// ideally this would be a lazy iterator - but need to think about lifetimes etc
    fn get_visible(&self) -> HashSet<(BlockCoordinates, &BlockType)> {
        let mut visible = HashSet::new();
        for (chunk_position, block_type) in self.blocks.iter() {
            if !self.mask.contains(chunk_position) {
                visible.insert((*chunk_position, *block_type));
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
    fn get_or_create(&mut self, coordinates: ChunkCoordinates) -> &HashChunk;
    fn at(&self, position: Position, radius: u8) -> Vec<(Position, &BlockType)>;
}

pub struct InMemoryWorld {
    generator: Box<dyn WorldGenerator>,
    chunks: HashMap<ChunkCoordinates, HashChunk>,
}

impl World for InMemoryWorld {
    fn new() -> InMemoryWorld {
        let seed = rand::random::<usize>();
        InMemoryWorld {
            generator: Box::new(worldgen::NaturalWorldGenerator::new(seed)),
            chunks: HashMap::new()
        }
    }

    fn get_or_create(&mut self, coordinates: ChunkCoordinates) -> &HashChunk {
        if self.chunks.contains_key(&coordinates) {
            return self.chunks.get(&coordinates).unwrap();
        } else {
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
    use block;
    use world::{get_position, position_to_chunk, Chunk, HashChunk, CHUNK_SIZE};

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
        let mut chunk = HashChunk::new();
        chunk.set([0, 0, 0].into(), &block::GRASS);
        assert_eq!(chunk.get([0, 0, 0].into()), Some(&block::GRASS));
    }
}
