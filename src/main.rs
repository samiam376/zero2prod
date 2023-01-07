use std::net::TcpListener;

use zero2prod::{build_router, run};

#[tokio::main]
async fn main() {
    let router = build_router();
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind");

    run(router, listener).await;
}
