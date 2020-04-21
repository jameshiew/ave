use std::thread;
use std::time::{Duration, Instant};

pub enum Action {
    Stop,
    Continue,
}

pub fn run<F>(mut callback: F)
where
    F: FnMut() -> Action,
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    // not really sure how to measure FPS accurately so call it TPS
    let mut ticks_per_second;
    let mut this_second = Duration::new(0, 0);
    let mut ticks_this_second = 0;

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };
        ticks_this_second += 1;

        let now = Instant::now();
        let time_passed = now - previous_clock;
        previous_clock = now;

        this_second += time_passed;
        if this_second > Duration::new(1, 0) {
            ticks_per_second = ticks_this_second;
            ticks_this_second = 0;
            this_second = Duration::new(0, 0);
            log::debug!("TPS: {}", ticks_per_second)
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
