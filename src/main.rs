use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero_to_prod::configuration::get_configuration;
use zero_to_prod::telemetry::{get_subscriber, init_subscriber};

use zero_to_prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero_to_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap_or_else(|_| {
        panic!(
            "Failed to bind to port: {}.",
            configuration.application_port
        )
    });
    run(listener, connection_pool)?.await
}
