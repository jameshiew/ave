use crate::block;
use crate::world::{get_position, Chunk, ChunkCoordinates, HashChunk, CHUNK_SIZE};
use log::debug;
use noise::{NoiseFn, Perlin, Seedable};
use rand::{Rng, SeedableRng, StdRng};

pub trait WorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> HashChunk;
}

/// Generates a flat world with no structures
///
/// Everything at and below world y=0 is ground, everything above is air
#[allow(dead_code)]
pub struct FlatWorldGenerator {}

impl FlatWorldGenerator {
    #[allow(dead_code)]
    pub fn new() -> FlatWorldGenerator {
        FlatWorldGenerator {}
    }
}

impl WorldGenerator for FlatWorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> HashChunk {
        let mut chunk = HashChunk::new();
        if coordinates[1] < 0 {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        chunk.set([x, y, z].into(), block::GRASS);
                    }
                }
            }
        }
        chunk
    }
}

/// A flat world generator with pillars of random height
#[allow(dead_code)]
pub struct RandomPillarsWorldGenerator {
    prng: StdRng,
}

#[allow(dead_code)]
impl RandomPillarsWorldGenerator {
    pub fn new(seed: usize) -> RandomPillarsWorldGenerator {
        let s: &[_] = &[seed];
        RandomPillarsWorldGenerator {
            prng: StdRng::from_seed(s),
        }
    }
}

impl WorldGenerator for RandomPillarsWorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> HashChunk {
        let mut chunk = HashChunk::new();
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
        chunk
    }
}

/// Generate a natural looking world
pub struct NaturalWorldGenerator {
    perlin: Perlin,
}

impl NaturalWorldGenerator {
    pub fn new(seed: u32) -> NaturalWorldGenerator {
        let mut generator = NaturalWorldGenerator {
            perlin: Perlin::new(),
        };
        generator.perlin = generator.perlin.set_seed(seed);
        debug!(
            "Using seed {} for NaturalWorldGenerator",
            generator.perlin.seed()
        );
        generator
    }
}

impl WorldGenerator for NaturalWorldGenerator {
    fn generate_chunk(&mut self, coordinates: ChunkCoordinates) -> HashChunk {
        let mut chunk = HashChunk::new();
        if coordinates[1] == 0 {
            // only create hills in ground chunks
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    // we need a height in the range [0, CHUNK_SIZE)
                    // https://www.redblobgames.com/maps/terrain-from-noise/ is a good source for tips
                    let position = get_position(&coordinates, [x, 0, z].into());
                    let height = self.perlin.get([(position.x * 0.015) as f64, (position.z * 0.015) as f64]);
                    // raise height to decent even power to so we get more flats and its nonnegative
                    let normalized_height: u8 = (height.powi(4) * (CHUNK_SIZE as f64)) as u8;
                    let mut blk = block::GRASS;
                    if normalized_height == 0 {
                        if height < 0.0 {
                            blk = block::SAND
                        }
                        chunk.set([x, 0, z].into(), blk);
                    } else {
                        for y in 0..normalized_height + 1 {
                            chunk.set([x, y, z].into(), block::DIRT);
                        }
                        // high peaks
                        for y in 20..normalized_height + 1 {
                            chunk.set([x, y, z].into(), block::STONE);
                        }
                    }
                }
            }
        }
        chunk
    }
}
