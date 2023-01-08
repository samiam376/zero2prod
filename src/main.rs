use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    startup::{build_router, run},
    telemetry,
};

#[tokio::main]
async fn main() {
    telemetry::initialize("info".into());

    let configuration = get_configuration().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("can't connect to database");

    let router = build_router(pool);
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind");

    run(router, listener).await;
}
