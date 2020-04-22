use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

pub fn initialize() -> Registry {
    Registry::new()
}
