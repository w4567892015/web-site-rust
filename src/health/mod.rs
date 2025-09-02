use actix_web::{web, get, HttpResponse, Responder};

pub fn init() -> actix_web::Scope {
  web::scope("/healthchecker")
    .service(health_checker_handler)
}

#[get("")]
async fn health_checker_handler() -> impl Responder {
  HttpResponse::Ok()
}
