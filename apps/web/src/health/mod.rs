mod controller;

use actix_web::web;

pub fn init() -> actix_web::Scope {
  web::scope("/healthchecker")
    .service(controller::health_checker_handler)
}


