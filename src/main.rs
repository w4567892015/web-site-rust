mod otel;
mod health;
mod api;

use std::{env, u16};

use tracing::instrument;
use tracing_actix_web::TracingLogger;
use actix_web::{App, HttpServer, middleware::Compat};
use actix_cors::Cors;
use actix_files::Files;

#[instrument]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let worker = env::var("WORKER_NUM")
    .unwrap_or("1".to_string())
    .parse::<usize>()
    .expect("WORKER_NUM must be a valid usize number");

  let host = env::var("HOST")
    .unwrap_or("0.0.0.0".to_string());

  let port = env::var("PORT")
    .unwrap_or("3000".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid u16 number");

  otel::init();

  println!("🚀 Server ready at http://{}:{} worker: {}", host, port, worker);

  HttpServer::new(move || {
    let _ = Cors::default()
      .allow_any_origin()
      .allow_any_header()
      .allow_any_method()
      .supports_credentials();
    App::new()
      .wrap(Compat::new(TracingLogger::default()))
      .service(Files::new("/assets", "./assets")
        .index_file("index.html")
        .use_etag(true)
        .use_last_modified(true)
      )
      .service(health::init())
      .service(api::init())
  })
  .workers(worker)
  .bind((host, port))?
  .run()
  .await
}
