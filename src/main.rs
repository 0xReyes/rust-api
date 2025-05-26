use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Hello from Rust API on Render!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
