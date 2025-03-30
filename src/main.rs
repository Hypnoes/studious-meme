mod config;
mod logging;
mod metrics;
mod routes;

use actix_web::{App, HttpServer, web};
use actix_web_prom::PrometheusMetricsBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let settings = config::load_config();

    // Initialize logging
    logging::initialize_logger(&settings.log_output);

    log::info!("Logger initialized");
    log::info!("Database DSN: {}", settings.database_dsn);
    log::info!("HTTP Port: {}", settings.http_port);
    log::info!("Log Output: {}", settings.log_output);

    log::info!("Starting server...");

    // Initialize Prometheus metrics middleware
    let default_prometheus_registry = prometheus::default_registry();
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .registry(default_prometheus_registry.clone())
        .build()
        .unwrap();

    // Start CPU usage monitoring
    metrics::start_cpu_usage_monitoring();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .route("/", web::get().to(routes::root))
            .route("/health", web::get().to(routes::health))
            .route("/metrics", web::get().to(routes::metrics))
    })
    .bind(("127.0.0.1", settings.http_port))?
    .run()
    .await
}
