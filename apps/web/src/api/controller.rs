use actix_web::{get, HttpResponse, Responder};

#[get("")]
async fn index() -> impl Responder {
  let result = super::service::index().await;
  HttpResponse::Ok().body(result)
}
