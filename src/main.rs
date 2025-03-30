use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use config::Config;
use flexi_logger::{FileSpec, LogSpecBuilder, Logger, WriteMode};
use prometheus::{Encoder, TextEncoder, register_gauge};
use serde::Deserialize;
use std::thread;
use std::time::Duration;
use sysinfo::System;

// Create a Prometheus gauge for CPU usage
lazy_static::lazy_static! {
    static ref CPU_USAGE: prometheus::Gauge = register_gauge!(
        "cpu_usage",
        "CPU usage percentage",
    ).unwrap();
}

// Struct to hold configuration settings
#[derive(Deserialize)]
struct AppConfig {
    database_dsn: String,
    http_port: u16,
    log_output: String, // "console", "file", or "both"
}

async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// Endpoint to return "ok!"
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

// Endpoint to expose Prometheus metrics
async fn metrics(prometheus: web::Data<PrometheusMetrics>) -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = prometheus.registry.gather();
    let metric_families = metric_families.as_slice();
    let mut buffer = Vec::new();
    encoder.encode(metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();
    // Convert to AppConfig
    let settings = settings.try_deserialize::<AppConfig>().unwrap();

    // Initialize logging
    let log_spec = LogSpecBuilder::new()
        .default(flexi_logger::LevelFilter::Info)
        .build();
    let _logger = match settings.log_output.as_str() {
        "console" => Logger::with(log_spec).log_to_stdout().start().unwrap(),
        "file" => Logger::with(log_spec)
            .log_to_file(FileSpec::default().directory("logs"))
            .rotate(
                flexi_logger::Criterion::Size(10_000_000), // Rotate when file size exceeds 10MB
                flexi_logger::Naming::Timestamps,
                flexi_logger::Cleanup::KeepLogFiles(5),
            )
            .write_mode(WriteMode::BufferAndFlush)
            .start()
            .unwrap(),
        "both" | _ => Logger::with(log_spec)
            .log_to_file(FileSpec::default().directory("logs"))
            .duplicate_to_stdout(flexi_logger::Duplicate::Info)
            .rotate(
                flexi_logger::Criterion::Size(10_000_000),
                flexi_logger::Naming::Timestamps,
                flexi_logger::Cleanup::KeepLogFiles(5),
            )
            .write_mode(WriteMode::BufferAndFlush)
            .start()
            .unwrap(),
    };

    log::info!("Logger initialized");
    log::info!("Database DSN: {}", settings.database_dsn);
    log::info!("HTTP Port: {}", settings.http_port);
    log::info!("Log Output: {}", settings.log_output);

    log::info!("Logger initialized");
    log::info!("Starting server...");

    // Initialize Prometheus metrics middleware
    let default_prometheus_registry = prometheus::default_registry();
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .registry(default_prometheus_registry.clone())
        .build()
        .unwrap();

    // Spawn a thread to simulate CPU usage updates
    thread::spawn(move || {
        let mut system = System::new_all(); // Create a System object to fetch system info
        loop {
            system.refresh_cpu_usage(); // Refresh CPU information
            let cpu_usage = system.global_cpu_usage(); // Get global CPU usage
            log::info!("Real CPU usage: {:.2}%", cpu_usage); // Log the CPU usage
            CPU_USAGE.set(cpu_usage as f64); // Update the Prometheus gauge
            thread::sleep(Duration::from_secs(5)); // Update every 5 seconds
        }
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health))
            .route("/metrics", web::get().to(metrics))
    })
    .bind(("127.0.0.1", settings.http_port))?
    .run()
    .await
}
