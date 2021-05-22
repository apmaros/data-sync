use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};
use actix_web::dev::Server as ActixServer;
use crate::server::api::*;
use crate::client::monzo::monzo_client::MonzoClient;
use std::sync::Arc;

pub struct ServerContext {
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) bank_client: Arc<MonzoClient>
}

struct AppState {
    pub(crate) mozno_client: Arc<MonzoClient>
}

pub async fn start_server(context: &ServerContext) -> std::io::Result<()> {
    let mozno_client = context.bank_client.clone();
    HttpServer::new(move || {
        App::new()
            .data(AppState { mozno_client })
            .service(index)
            .service(healthz)
            .service(auth)
    }).bind(
        format!("{}:{}", context.host, context.port)
    )?.run().await
}
