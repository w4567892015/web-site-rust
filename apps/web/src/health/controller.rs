use actix_web::{get, HttpResponse, Responder};

#[get("")]
async fn health_checker_handler() -> impl Responder {
  HttpResponse::Ok()
}
