use world::{ChunkCoordinates, Chunk, CHUNK_SIZE};
use rand::{Rng, StdRng, SeedableRng};

pub trait WorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> Chunk;
}

/// Generates a flat world with no structures
///
/// Everything at and below world y=0 is ground, everything above is air
pub struct FlatWorldGenerator {}

impl FlatWorldGenerator {
    pub fn new() -> FlatWorldGenerator {
        FlatWorldGenerator {}
    }
}

impl WorldGenerator for FlatWorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> Chunk {
        let mut chunk = Chunk::new();
        if coordinates[1] < 0 {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        chunk.set([x, y, z].into(), 0);
                    }
                }
            }
        }
        return chunk;
    }
}

/// A flat world generator with pillars of random height
pub struct RandomPillarsWorldGenerator {
    prng: StdRng,
}

impl RandomPillarsWorldGenerator {
    pub fn new(seed: usize) -> RandomPillarsWorldGenerator {
        let s: &[_] = &[seed];
        RandomPillarsWorldGenerator { prng: StdRng::from_seed(s) }
    }
}

impl WorldGenerator for RandomPillarsWorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> Chunk {
        let mut chunk = Chunk::new();
        if coordinates[1] == 0 {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    chunk.set([x, 0, z].into(), 0);
                }
            }
        }

        let number_of_pillars = self.prng.gen_range(1, 8);
        for _ in 0..number_of_pillars {
            let pillar_x = self.prng.gen_range(0, CHUNK_SIZE - 1);
            let pillar_z = self.prng.gen_range(0, CHUNK_SIZE - 1);
            let pillar_height = self.prng.gen_range(0, CHUNK_SIZE);
            for y in 0..pillar_height {
                chunk.set([pillar_x, y, pillar_z].into(), 1);
            }
        }
        return chunk;
    }
}
