#[macro_use]
extern crate log;
extern crate redis;

mod client;
mod util;
mod error;
mod server;

use env_logger;
use crate::client::monzo::monzo_client::MonzoClient;

use crate::server::server::{ServerContext, start_server};
use crate::util::exit_with_error;
use crate::error::GenError;

#[actix_web::main]
async fn main() {
    env_logger::init();
    info!("Starting Data Sync App");
    let monzo_client = match MonzoClient::new() {
        Ok(c) => c,
        Err(err) => exit_with_error(err)
    };

    let rclient = redis::Client::open("redis://127.0.0.1/").unwrap();

    let server_context = ServerContext {
        host: "127.0.0.1".to_string(),
        port: 8050.to_string(),
        bank_client: monzo_client,
        redis_client: rclient
    };

    match start_server(server_context.clone()).await {
        Ok(()) => info!("Server started on http://{}:{}",
                        server_context.host,
                        server_context.port
        ),
        Err(err) => {
            exit_with_error(GenError::from(err));
        }
    }
}
