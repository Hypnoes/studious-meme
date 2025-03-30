use actix_web::{HttpResponse, Responder, web};
use actix_web_prom::PrometheusMetrics;
use prometheus::{Encoder, TextEncoder};

pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn metrics(prometheus: web::Data<PrometheusMetrics>) -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = prometheus.registry.gather();
    let metric_families = metric_families.as_slice();
    let mut buffer = Vec::new();
    encoder.encode(metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
