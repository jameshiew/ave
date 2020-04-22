use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::Sender;

pub enum Action {
    Stop,
    Continue,
}

// not really sure how to measure FPS accurately so call it TPS
pub fn run<F>(tps: Sender<f64>, mut each_tick: F)
where
    F: FnMut() -> Action,
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let mut this_second = Duration::new(0, 0);
    let mut ticks_this_second = 0;

    loop {
        match each_tick() {
            Action::Stop => break,
            Action::Continue => (),
        };
        ticks_this_second += 1;

        let now = Instant::now();
        let time_passed = now - previous_clock;
        previous_clock = now;

        this_second += time_passed;
        if this_second > Duration::new(1, 0) {
            tps.send(ticks_this_second as f64).unwrap();
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
