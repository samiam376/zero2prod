use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{health_check::health, subscriptions::subscribe};

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
