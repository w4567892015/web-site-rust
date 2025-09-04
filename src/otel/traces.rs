use std::time::Duration;

use opentelemetry::global;
use opentelemetry_sdk::{propagation::TraceContextPropagator};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_otlp::{WithExportConfig, SpanExporter};

pub fn init_traces() -> SdkTracerProvider {
  global::set_text_map_propagator(TraceContextPropagator::new());

  let exporter = SpanExporter::builder()
    .with_tonic()
    .with_endpoint(super::get_endpoint())
    .with_timeout(Duration::from_secs(3))
    .build()
    .expect("Failed to create span exporter");

  SdkTracerProvider::builder()
    .with_batch_exporter(exporter)
    .with_resource(super::RESOURCE.clone())
    .build()
}
