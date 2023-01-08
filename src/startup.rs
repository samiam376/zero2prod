use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::info;


use crate::routes::{health_check::health, subscriptions::subscribe};

pub fn build_router(connection_pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(connection_pool)
}

pub async fn run(router: Router, listener: std::net::TcpListener) {

    let addr = listener.local_addr().unwrap();
    info!("listening on: {}", addr);

    axum::Server::from_tcp(listener)
        .expect("failed to build server")
        .serve(router.into_make_service())
        .await
        .unwrap();
}
