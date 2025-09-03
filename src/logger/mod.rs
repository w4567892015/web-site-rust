use std::{env};
use std::time::Duration;

use tracing::{subscriber::set_global_default};
use tracing_subscriber::{fmt, EnvFilter, Registry, prelude::*};
use tracing_log::LogTracer;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_opentelemetry::OpenTelemetryLayer;

use opentelemetry::trace::TracerProvider;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, Resource};
use opentelemetry_semantic_conventions::resource;

pub fn init() -> opentelemetry_sdk::trace::SdkTracerProvider{
  let service_name = env::var("APP_NAME")
    .unwrap_or("undefined".to_string());

  let endpoint = env::var("OTEL_EXPORTER_ENDPOINT")
    .unwrap_or("http://localhost:4317".to_string());

  global::set_text_map_propagator(TraceContextPropagator::new());

  let exporter = opentelemetry_otlp::SpanExporter::builder()
    .with_tonic()
    .with_endpoint(endpoint)
    .with_timeout(Duration::from_secs(3))
    .build()
    .expect("OTLP exporter failed");

  let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
    .with_batch_exporter(exporter)
    .with_resource(
      Resource::builder()
      .with_attribute(KeyValue::new(resource::SERVICE_NAME, service_name.to_string()))
      .with_attribute(KeyValue::new(resource::SERVICE_VERSION, "0.1.0".to_string()))
      .build())
    .build();

  let tracer = provider.tracer(service_name.to_string());

  let env_filter = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();

   let fmt_layer = fmt::Layer::default()
    .with_target(true)
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_ansi(true)
    .compact();

  let subscriber = Registry::default()
    .with(env_filter)
    .with(fmt_layer)
    .with(OpenTelemetryLayer::new(tracer))
    .with(JsonStorageLayer)
    .with(BunyanFormattingLayer::new(service_name.to_string(), std::io::stdout));

  LogTracer::init().expect("Failed to set logger");
  set_global_default(subscriber).expect("Failed to set subscriber");

  provider
}
