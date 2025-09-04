mod controller;
mod service;

use actix_web::web;

pub fn init() -> actix_web::Scope {
  web::scope("/api")
    .service(controller::index)
}
