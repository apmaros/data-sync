use actix_web::{get, Responder, HttpResponse, web};
use serde::{ Serialize, Deserialize};
use crate::server::server::ServerContext;
use actix_session::Session;

#[get("/")]
pub async fn index(session: Session) -> impl Responder {
    match session.get::<String>("monzotoken").unwrap() {
        Some(r) => info!("token={}", r),
        None => info!("Token is not set") }

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

#[get("login-monzo")]
pub fn login_monzo(data: web::Data<ServerContext>) -> HttpResponse {
    let (url, _) = &data.bank_client.get_auth_url();
    info!("Redirecting user to {}", url);

    HttpResponse::PermanentRedirect().header("Location", url.as_str()).finish()
}

#[get("/home")]
pub async fn auth(
    auth: web::Query<AuthParam>,
    data: web::Data<ServerContext>,
    session: Session
) -> impl Responder {
    let client = &data.bank_client;
    let code = auth.code.to_string();

    match client.get_token(code) {
        Ok(token) => {
            info!("user successfully authenticated");
            // todo set error handling
            session.set("monzotoken", token).unwrap();
        },
        Err(err) => println!("failed to get access token {}", err)
    }
    web::Json(
        Healthz{status: "ok".to_string()}
    )
}


