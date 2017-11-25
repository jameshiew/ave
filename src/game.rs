use world;
use world::World;

/// TODO: should be along the lines of `world: W where W: world::World`
pub struct Game {
    pub world: world::InMemoryWorld,
}

impl Game {
    pub fn new() -> Game {
        Game { world: world::InMemoryWorld::new() }
    }
}