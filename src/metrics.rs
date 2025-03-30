use prometheus::{Gauge, register_gauge};
use std::thread;
use std::time::Duration;
use sysinfo::System;

lazy_static::lazy_static! {
    pub static ref CPU_USAGE: Gauge = register_gauge!(
        "cpu_usage",
        "CPU usage percentage",
    ).unwrap();
}

pub fn start_cpu_usage_monitoring() {
    thread::spawn(move || {
        let mut system = System::new_all();
        loop {
            system.refresh_cpu_usage();
            let cpu_usage = system.global_cpu_usage();
            log::info!("Real CPU usage: {:.2}%", cpu_usage);
            CPU_USAGE.set(cpu_usage as f64);
            // Update every 5 seconds
            thread::sleep(Duration::from_secs(5));
        }
    });
}
