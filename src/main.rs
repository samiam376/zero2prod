use axum::{response::Html, routing::get, Router};
use http::StatusCode;
use std::net::SocketAddr;

async fn health() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
