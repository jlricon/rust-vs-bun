use salvo::prelude::*;
#[handler]
async fn hello_world(res: &mut Response) {
    res.render("Hello world!");
}
#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    let listener = TcpListener::bind("127.0.0.1:3000");
    Server::new(listener).serve(router).await;
}