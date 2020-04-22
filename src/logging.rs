use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};

use std::sync::Once;

static INITIALIZE: Once = Once::new();

pub fn initialize() {
    INITIALIZE.call_once(|| {
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Stdout,
        )
        .unwrap()])
        .unwrap()
    })
}
