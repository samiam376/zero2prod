use axum::{response::Html, routing::{get, post}, Router, Form};
use http::StatusCode;
use serde::Deserialize;

async fn health() -> StatusCode {
    StatusCode::OK
}


#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

async fn subscribe(form: Form<FormData>) -> StatusCode {
    StatusCode::OK
}

pub fn build_router() -> Router {
    Router::new()
        .route("/health_check", get(health))
        .route("/subscriptions", post(subscribe))
}

pub async fn run(router: Router, listener: std::net::TcpListener) {
    axum::Server::from_tcp(listener)
        .expect("failed to build server")
        .serve(router.into_make_service())
        .await
        .unwrap();
}
