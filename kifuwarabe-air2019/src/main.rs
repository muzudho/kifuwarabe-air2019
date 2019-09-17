/*
# Current directory
cd ./kifuwarabe-air2019

# - Log level.
#$env:RUST_LOG = "trace"
#$env:RUST_LOG = "debug"
$env:RUST_LOG = "info"
#$env:RUST_LOG = "warn"
#$env:RUST_LOG = "error"

# - Log redirect.
cargo run 2>> ./kifuwarabe-air2019.log
 */

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init();

    println!("Hello, println!");
    trace!("Hello, trace!");
    debug!("Hello, debug!");
    info!("Hello, info!");
    warn!("Hello, warn!");
    error!("Hello, error!");
}
