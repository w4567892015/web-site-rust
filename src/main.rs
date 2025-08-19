mod index;

use std::env;
use actix_web::{App, HttpServer, middleware::Logger};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let host = env::var("HOST")
    .unwrap_or("0.0.0.0".to_string());

  let port = env::var("PORT")
    .unwrap_or("3000".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid u16 number");

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  HttpServer::new(move || {
    App::new()
    .wrap(Logger::default())
    .service(index::init())
  })
  .bind((host, port))?
  .run()
  .await
}
