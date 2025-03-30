use flexi_logger::{FileSpec, LogSpecBuilder, Logger, WriteMode};

pub fn initialize_logger(log_output: &str) {
    let log_spec = LogSpecBuilder::new()
        .default(flexi_logger::LevelFilter::Info)
        .build();

    match log_output {
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
}
