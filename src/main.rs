use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(move || App::new().route("/", web::get().to(root)))
        .bind(("127.0.0.1", 9000))?
        .run()
        .await
}
