use actix_web::{web, App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(|| async { "Hello World!" })))
        .workers(4)
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
