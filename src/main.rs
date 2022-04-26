use sqlx::PgPool;
use std::net::TcpListener;
use servertest::startup::run;
use servertest::configuration::get_configuration;
use servertest::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("servertest".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // We have removed the hard-coded '8000' - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
