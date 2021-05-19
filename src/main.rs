mod client;
mod util;
mod error;

extern crate log;

use env_logger;
use log::{info};

fn main() {
    env_logger::init();
    info!("Starting Data Sync App");

    print!("{}", "ontext")
}
