#[macro_use]
extern crate lazy_static;

mod client;
mod util;
mod error;
mod server;

extern crate log;

use env_logger;
use log::{info, error};
use crate::client::monzo::monzo_client::MonzoClient;
use crate::client::monzo::monzo_client::MONZO_CLIENT;

use crate::server::server::{ServerContext, start_server};
use std::process::exit;
use std::sync::Arc;

#[actix_web::main]
async fn main() {
    info!("auth_url={}", MONZO_CLIENT.get_auth_url().0);
    let a = MONZO_CLIENT.get_auth_url().0;
    env_logger::init();
    let c = MonzoClient::new().expect("Boo");
    let rc = Arc::new(c);
    info!("Starting Data Sync App");
    let server_config = ServerContext {
        host: "127.0.0.1".to_string(),
        port: 8080.to_string(),
        bank_client: rc
    };

    match start_server(&server_config).await {
        Ok(()) => info!("Server started on http://{}:{}",
                        server_config.host,
                        server_config.port
        ),
        Err(err) => {
            error!("failed to start server due to {}", err);
            exit(1);
        }
    }
}
