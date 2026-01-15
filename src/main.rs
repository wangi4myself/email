use email::configuration::get_configuration;
use email::startup::run;
use email::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email".into(), "info".into(),std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to the database.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Listening on port {}", configuration.application_port);
    run(listener, connection_pool)?.await
}
