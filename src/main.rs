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
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("can't connect to database");

    let router = build_router(pool);
    let addr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(addr).expect("failed to bind");

    run(router, listener).await;
}
