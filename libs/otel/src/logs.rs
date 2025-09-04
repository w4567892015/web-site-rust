use std::time::Duration;

use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_otlp::{WithExportConfig, LogExporter};

pub fn init_logs() -> SdkLoggerProvider {
  let exporter = LogExporter::builder()
    .with_tonic()
    .with_endpoint(super::get_endpoint())
    .with_timeout(Duration::from_secs(3))
    .build()
    .expect("Failed to create log exporter");

  SdkLoggerProvider::builder()
    .with_resource(super::RESOURCE.clone())
    .with_batch_exporter(exporter)
    .build()
}
