use email::configuration::{get_configuration, Application};
use email::telemetry::{get_subscriber, init_subscriber};
// use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
    // let connection_pool = PgPoolOptions::new()
    //     .acquire_timeout(std::time::Duration::from_secs(2))
    //     .connect_lazy_with(configuration.database.with_db());

    // let sender_email = configuration
    //     .email_client
    //     .sender_email()
    //     .expect("Invalid sender email address in configuration.");

    // let timeout = configuration.email_client.timeout_duration();

    // let email_client = EmailClient::new(
    //     configuration.email_client.base_url,
    //     sender_email,
    //     configuration.email_client.authorization_token,
    //     timeout,
    // );

    // let address = format!(
    //     "{}:{}",
    //     configuration.application.host, configuration.application.port
    // );
    // let listener = TcpListener::bind(address)?;
    // println!("Listening on port {}", configuration.application.port);
    // run(listener, connection_pool, email_client)?.await?;
}
