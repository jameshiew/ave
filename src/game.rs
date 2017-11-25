use world;
use world::World;

/// TODO: should be along the lines of `world: W where W: world::World`
pub struct Game {
    pub world: world::InMemoryWorld,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game { world: world::InMemoryWorld::new() };
        for x in -3..3 + 1 {
            for y in -3..3 + 1 {
                for z in -3..3 + 1 {
                    game.world.get_or_create([x, y, z].into());
                }
            }
        }
        game
    }
}