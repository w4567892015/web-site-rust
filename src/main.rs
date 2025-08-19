use std::env;
use actix_web::{get, App, HttpServer, Responder, middleware::Logger};

#[get("/")]
async fn index() -> impl Responder {
  "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let host = env::var("HOST")
    .unwrap_or_else(|_| "0.0.0.0".to_string());

  let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid u16 number");

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  HttpServer::new(|| {
    App::new()
    .wrap(Logger::default())
    .service(index)
  })
  .bind((host, port))?
  .run()
  .await
}
