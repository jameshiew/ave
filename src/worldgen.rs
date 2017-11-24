use world::{ChunkCoordinates, Chunk, CHUNK_SIZE};

pub trait WorldGenerator {
    fn generate_chunk(&self, coordinates: ChunkCoordinates) -> Chunk;
}

/// Simple world generator which generates a flat surface for chunks at y=0
pub struct FlatWorldGenerator {}

impl FlatWorldGenerator {
    pub fn new() -> FlatWorldGenerator {
        FlatWorldGenerator {}
    }
}

impl WorldGenerator for FlatWorldGenerator {

    fn generate_chunk(&self, coordinates: ChunkCoordinates) -> Chunk {
        let mut chunk = Chunk::new();
        if coordinates[1] == 0 {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    chunk.set([x, 0, z].into(), 0);
                }
            }
        }
        return chunk;
    }
}

/// A flat world generator with pillars of random height
pub struct RandomPillarsWorldGenerator {
    seed: i32,
}

impl RandomPillarsWorldGenerator {
    pub fn new(seed: i32) -> RandomPillarsWorldGenerator {
        RandomPillarsWorldGenerator { seed }
    }
}
