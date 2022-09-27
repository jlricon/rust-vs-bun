//! A Http server returns Hello World String as Response.

use std::io;

use xitca_http::{
    http::{const_header_value::TEXT_UTF8, header::CONTENT_TYPE, Response},
    HttpServiceBuilder, Request, RequestBody, ResponseBody,
};
use xitca_service::fn_service;

fn main() -> io::Result<()> {
    // construct server
    xitca_server::Builder::new()
        // bind to both tcp and udp addresses where a single service would handle http/1/2/3 traffic.
        .bind("hello-world", "127.0.0.1:3000", move || {
            HttpServiceBuilder::new(fn_service(handler)).with_logger()
        })?
        .build()
        .wait()
}

async fn handler(
    _: Request<RequestBody>,
) -> Result<Response<ResponseBody>, Box<dyn std::error::Error>> {
    let res = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, TEXT_UTF8)
        .body("Hello World!".into())?;
    Ok(res)
}
