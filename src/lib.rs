use axum::{response::Html, routing::get, Router};
use http::StatusCode;

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/health_check", get(health))
}

pub async fn run(router: Router, listener: std::net::TcpListener) {
    axum::Server::from_tcp(listener)
        .expect("failed to build server")
        .serve(router.into_make_service())
        .await
        .unwrap();
}
