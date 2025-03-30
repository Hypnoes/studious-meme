use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use config::Config;
use flexi_logger::{FileSpec, LogSpecBuilder, Logger, WriteMode};
use serde::Deserialize;

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

    // Start the HTTP server
    HttpServer::new(move || App::new().route("/", web::get().to(root)))
        .bind(("127.0.0.1", settings.http_port))?
        .run()
        .await
}
