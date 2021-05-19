mod client;
mod util;
mod error;

extern crate log;

use env_logger;
use log::{info};
use crate::client::monzo::monzo_client::MonzoClient;


fn main() {
    env_logger::init();
    info!("Starting Data Sync App");
}
