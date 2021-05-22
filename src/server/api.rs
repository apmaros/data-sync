use actix_web::{get, Responder, HttpResponse, web, HttpRequest};
use serde::{ Serialize, Deserialize};
use crate::client::monzo::monzo_client::MONZO_CLIENT;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("data-sync is running")
}

#[derive(Serialize)]
struct Healthz {
    status: String
}

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    web::Json(
        Healthz{status: "ok".to_string()}
    )
}

#[derive(Deserialize)]
pub struct AuthParam {
    code: String
}

#[get("/auth")]
pub async fn auth(auth: web::Query<AuthParam>) -> impl Responder {
    match MONZO_CLIENT.get_token(auth.code.to_string()) {
        Ok(_) => println!("Good job"),
        Err(err) => println!("uups {}", err)
    }
    web::Json(
        Healthz{status: "ok".to_string()}
    )
}


