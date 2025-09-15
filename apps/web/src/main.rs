use otel;
mod health;
mod api;

use std::{env, u16};

use tracing::instrument;
use tracing_actix_web::TracingLogger;
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use actix_files::Files;

#[instrument]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let assets_dir = env::current_exe()?
    .ancestors().nth(1).unwrap().to_path_buf()
    .join("assets");

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

  let providers = otel::init();

  println!("🚀 Server ready at http://{}:{} worker: {}", host, port, worker);

  HttpServer::new(move || {
    let _ = Cors::default()
      .allow_any_origin()
      .allow_any_header()
      .allow_any_method()
      .supports_credentials();
    App::new()
      .wrap(TracingLogger::default())
      .service(Files::new("/assets", assets_dir.as_path())
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
  .await?;

  providers.expect("OTEL Provider Not Found!").showdown();

  Ok(())
}
