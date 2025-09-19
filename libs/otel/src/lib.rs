mod logs;
mod traces;
mod metrics;

use std::env;
use std::sync::LazyLock;

use tracing::{subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, EnvFilter, Registry, prelude::*};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_opentelemetry::OpenTelemetryLayer;

use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_semantic_conventions::resource;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::Resource;

use opentelemetry_sdk::{logs::SdkLoggerProvider, trace::SdkTracerProvider, metrics::SdkMeterProvider};

fn get_enable_otel_logger() -> bool {
  env::var("ENABLE_OTEL_LOGGER")
    .unwrap_or("false".to_string())
    .parse::<bool>()
    .expect("ENABLE_OTEL_LOGGER must be a valid boolean")
}

fn get_enable_otel_tracer() -> bool {
  env::var("ENABLE_OTEL_TRACER")
    .unwrap_or("false".to_string())
    .parse::<bool>()
    .expect("ENABLE_OTEL_TRACER must be a valid boolean")
}

fn get_enable_otel_meter() -> bool {
  env::var("ENABLE_OTEL_METER")
    .unwrap_or("false".to_string())
    .parse::<bool>()
    .expect("ENABLE_OTEL_METER must be a valid boolean")
}

fn get_service_name() -> String {
  env::var("SERVICE_NAME")
    .unwrap_or("undefined".to_string())
}

fn get_endpoint() -> String {
  env::var("OTEL_EXPORTER_ENDPOINT")
    .unwrap_or("http://localhost:4317".to_string())
}

static RESOURCE: LazyLock<Resource> = LazyLock::new(|| {
  Resource::builder()
    .with_attribute(KeyValue::new(resource::SERVICE_NAME, get_service_name()))
    .with_attribute(KeyValue::new(resource::SERVICE_VERSION, "0.1.0".to_string()))
    .build()
});

pub struct Providers {
  logger_provider: SdkLoggerProvider,
  tracer_provider: SdkTracerProvider,
  meter_provider: Option<SdkMeterProvider>,
}

impl Providers {
  pub fn showdown(self) {
    let _ = self.logger_provider.shutdown();
    let _ = self.tracer_provider.shutdown();
    if self.meter_provider.is_some() {
      let _ = self.meter_provider.expect("REASON").shutdown();
    }
  }
}

pub fn init() -> Option<Providers> {
  let env_filter = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();

  let fmt_layer = fmt::Layer::default()
    .with_target(true)
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_ansi(true)
    .compact();

  let logger_filter = EnvFilter::new("INFO")
    .add_directive("hyper=off".parse().unwrap())
    .add_directive("opentelemetry=off".parse().unwrap())
    .add_directive("tonic=off".parse().unwrap())
    .add_directive("h2=off".parse().unwrap())
    .add_directive("reqwest=off".parse().unwrap());

  let logger_provider = logs::init_logs();
  let logger_layer = OpenTelemetryTracingBridge::new(&logger_provider).with_filter(logger_filter);

  let tracer_provider = traces::init_traces();
  let tracer_layer = tracer_provider.tracer(get_service_name());

  let mut meter_provider = None;
  if get_enable_otel_meter() {
    meter_provider = metrics::init_metrics();
  }

  let subscriber = Registry::default()
    .with(env_filter)
    .with(fmt_layer)
    .with(if get_enable_otel_logger() {
      Some(logger_layer)
    } else {
      None
    })
    .with( if get_enable_otel_tracer() {
      Some(OpenTelemetryLayer::new(tracer_layer))
    } else {
      None
    })
    .with(JsonStorageLayer)
    .with(BunyanFormattingLayer::new(get_service_name(), std::io::stdout))
    .with(fmt::layer());

  LogTracer::init().expect("Failed to set logger");
  set_global_default(subscriber).expect("Failed to set subscriber");

  Some(Providers { logger_provider, tracer_provider, meter_provider })
}
