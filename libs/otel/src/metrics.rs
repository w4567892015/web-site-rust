use std::time::Duration;

use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_otlp::{WithExportConfig, MetricExporter};

pub fn init_metrics() -> Option<SdkMeterProvider> {
  let exporter = MetricExporter::builder()
    .with_tonic()
    .with_endpoint(super::get_endpoint())
    .with_timeout(Duration::from_secs(3))
    .build()
    .expect("Failed to create metric exporter");

  Some(SdkMeterProvider::builder()
    .with_periodic_exporter(exporter)
    .with_resource(super::RESOURCE.clone())
    .build())
}
