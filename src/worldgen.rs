use world::{ChunkCoordinates, BlockCoordinates, Chunk, CHUNK_SIZE, get_position};
use rand::{Rng, StdRng, SeedableRng};
use noise::{NoiseModule, Perlin, Point2};
use std;
use block;

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
                        chunk.set([x, y, z].into(), block::GRASS);
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
                    chunk.set([x, 0, z].into(), block::GRASS);
                }
            }
        }

        let number_of_pillars = self.prng.gen_range(1, 8);
        for _ in 0..number_of_pillars {
            let pillar_x = self.prng.gen_range(0, CHUNK_SIZE - 1);
            let pillar_z = self.prng.gen_range(0, CHUNK_SIZE - 1);
            let pillar_height = self.prng.gen_range(0, CHUNK_SIZE);
            for y in 0..pillar_height {
                chunk.set([pillar_x, y, pillar_z].into(), block::DIRT);
            }
        }
        return chunk;
    }
}

/// Generate a natural looking world
pub struct NaturalWorldGenerator {
    perlin: Perlin,
}

impl NaturalWorldGenerator {
    pub fn new() -> NaturalWorldGenerator {
        NaturalWorldGenerator { perlin: Perlin::new() }
    }
}

impl WorldGenerator for NaturalWorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> Chunk {
        let mut chunk = Chunk::new();
        if coordinates[1] == 0 {  // only create hills in ground chunks
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    // we need a height in the range [0, CHUNK_SIZE)
                    // https://www.redblobgames.com/maps/terrain-from-noise/ is a good source for tips
                    let position = get_position(&coordinates, &[x, 0, z].into());
                    let mut height = self.perlin.get([position.x * 0.05, position.z * 0.05]).abs();
                    // raise to a power to so we get more 'flat' areas
                    height = height.powi(5);
                    let normalized_height: u8 = (height * (CHUNK_SIZE as f32)) as u8;
                    if normalized_height == 0 {
                        chunk.set([x, 0, z].into(), block::GRASS);
                    } else {
                        for y in 0..normalized_height + 1 {
                            chunk.set([x, y, z].into(), block::DIRT);
                        }
                    }
                }
            }
        }
        chunk
    }
}
