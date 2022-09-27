// Config from https://github.com/TechEmpower/FrameworkBenchmarks/blob/master/frameworks/Rust/actix/src/main_web.rs
use jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
use std::time::Duration;

use actix_http::{HttpService, KeepAlive};
use actix_service::map_config;
use actix_web::{
    dev::{AppConfig, Server},
    http::{
        header::{HeaderValue, CONTENT_TYPE, SERVER},
        StatusCode,
    },
    web::{self, Bytes},
    App, HttpResponse,
};
async fn plaintext() -> HttpResponse<Bytes> {
    let mut res = HttpResponse::with_body(StatusCode::OK, Bytes::from_static(b"Hello World!"));
    res.headers_mut()
        .insert(SERVER, HeaderValue::from_static("A"));
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    res
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started HTTP server: 127.0.0.1:3000");

    // start http server
    Server::build()
        .backlog(1024)
        .bind("tfb-actix-web", "0.0.0.0:3000", || {
            HttpService::build()
                .keep_alive(KeepAlive::Os)
                .client_request_timeout(Duration::ZERO)
                .h1(map_config(
                    App::new().service(web::resource("/").to(plaintext)),
                    |_| AppConfig::default(),
                ))
                .tcp()
        })?
        .workers(8)
        .run()
        .await
}
