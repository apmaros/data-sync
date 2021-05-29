use actix_web::{
    App,
    HttpServer
};
use actix_session::{
    CookieSession
};
use crate::server::api::*;
use crate::client::monzo::monzo_client::MonzoClient;
use redis::Client as RedisClient;

#[derive(Clone, Debug)]
pub struct ServerContext {
    pub host: String,
    pub port: String,
    pub bank_client: MonzoClient,
    pub redis_client: RedisClient
}

pub async fn start_server(context: ServerContext) -> std::io::Result<()> {
    let url = format!("{}:{}", context.host, context.port);
    HttpServer::new( move || {
        App::new()
            .data(context.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(index)
            .service(healthz)
            .service(auth)
            .service(login_monzo)
    }).bind(url)?.run().await
}
