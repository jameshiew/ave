use crate::{camera, default, world};
use std::thread;
use std::time::{Duration, Instant};
use world::World;

/// TODO: should be along the lines of `world: W where W: world::World`
pub struct Game {
    pub world: world::InMemoryWorld,
    pub camera: camera::Camera,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: world::InMemoryWorld::new(),
            camera: camera::Camera::new(),
        }
    }
    pub fn tick(&mut self) {
        {
            self.camera.update();
            // generate chunks as we move the camera
            let chunk_coords = world::position_to_chunk(&self.camera.position);
            let cx = chunk_coords.x;
            let cy = chunk_coords.y;
            let cz = chunk_coords.z;
            for x in (cx - default::RENDER_DISTANCE_I32)..(cx + default::RENDER_DISTANCE_I32) {
                for y in (cy - default::RENDER_DISTANCE_I32)..(cy + default::RENDER_DISTANCE_I32) {
                    for z in
                        (cz - default::RENDER_DISTANCE_I32)..(cz + default::RENDER_DISTANCE_I32)
                    {
                        self.world.get_or_create([x, y, z].into());
                    }
                }
            }
        }
    }
}

pub struct Ticker {
    tps: prometheus::Gauge,
}

impl Ticker {
    pub fn new() -> Ticker {
        Ticker {
            tps: prometheus::Gauge::new("ticks_per_seconds", "Ticks per second").unwrap(),
        }
    }
    pub fn run<F>(&self, mut each_tick: F)
    where
        F: FnMut() -> bool,
    {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();
        let mut this_second = Duration::new(0, 0);
        let mut ticks_this_second = 0;

        loop {
            let should_continue = each_tick();
            if !should_continue {
                break;
            }
            ticks_this_second += 1;

            let now = Instant::now();
            let time_passed = now - previous_clock;
            previous_clock = now;

            this_second += time_passed;
            if this_second > Duration::new(1, 0) {
                self.tps.set(ticks_this_second as f64);
                ticks_this_second = 0;
                this_second = Duration::new(0, 0);
            }

            accumulator += time_passed;

            let fixed_time_stamp = Duration::new(0, 16_666_667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                // if you have a game, update the state here
            }

            thread::sleep(fixed_time_stamp - accumulator);
        }
    }
    pub fn get_tps(&self) -> f64 {
        self.tps.get()
    }
}
