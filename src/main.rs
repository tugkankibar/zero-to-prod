use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero_to_prod::configuration::get_configuration;
use zero_to_prod::telemetry::{get_subscriber, init_subscriber};

use zero_to_prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero_to_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(30))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).unwrap_or_else(|_| {
        panic!(
            "Failed to bind to: {}:{}.",
            configuration.application.host, configuration.application.port
        )
    });
    run(listener, connection_pool)?.await
}
