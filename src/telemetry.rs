use tracing_subscriber::EnvFilter;

pub fn initialize(env_filter: String) {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter)),
        )
        .init();
}
