use email::configuration::get_configuration;
use email::startup::run;
use email::telemetry::{get_subscriber, init_subscriber};
// use secrecy::ExposeSecret;
use email::email_client::EmailClient;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    // 调试：打印数据库连接信息（不包含密码）
    // tracing::info!(
    //     "Database config: host={}, port={}, user={}, db={}, ssl={}",
    //     configuration.database.host,
    //     configuration.database.port,
    //     configuration.database.username,
    //     configuration.database.database_name,
    //     configuration.database.require_ssl
    // );

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender_email()
        .expect("Invalid sender email address in configuration.");

    let timeout = configuration.email_client.timeout_duration();

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    println!("Listening on port {}", configuration.application.port);
    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
