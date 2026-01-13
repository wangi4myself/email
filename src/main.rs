use email::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use email::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database.");
    
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Listening on port {}", configuration.application_port);
    run(listener, connection_pool)?.await
}