mod controller;
mod service;

use actix_web::web;

pub fn init() -> actix_web::Scope {
  web::scope("")
    .service(controller::index)
}
