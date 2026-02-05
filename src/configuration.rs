use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::startup::{get_connection_pool, run};
use actix_web::dev::Server;
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::net::TcpListener;

/// 应用程序实例，封装了 HTTP 服务器和端口信息
/// 负责构建和运行整个 Web 应用
pub struct Application {
    /// 服务器监听的实际端口号
    port: u16,
    /// Actix-web 服务器实例
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

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
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;
        Ok(Self { port, server })
        // run(listener, connection_pool, email_client)
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// 应用程序的根配置结构
/// 从 configuration/base.yaml + 环境配置文件反序列化而来
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    /// 数据库连接配置
    pub database: DatabaseSettings,
    /// HTTP 服务器配置
    pub application: ApplicationSettings,
    /// 邮件客户端配置
    pub email_client: EmailClientSettings,
}

/// 邮件服务客户端配置
/// 用于配置 Postmark 等邮件服务提供商的连接参数
#[derive(serde::Deserialize, Clone)]
pub struct EmailClientSettings {
    /// 邮件服务 API 地址，如 "https://api.postmarkapp.com"
    pub base_url: String,
    /// 发件人邮箱地址
    pub sender_email: String,
    /// API 授权令牌，使用 Secret 包装以防止日志泄露
    pub authorization_token: Secret<String>,
    /// 请求超时时间（毫秒）
    pub timeout_milliseconds: u64,
}

impl EmailClientSettings {
    pub fn sender_email(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }

    pub fn timeout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

/// HTTP 服务器配置
#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    /// 监听端口，支持从字符串反序列化（兼容环境变量）
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// 监听地址，如 "127.0.0.1" 或 "0.0.0.0"
    pub host: String,
    pub base_url: String,
}

/// PostgreSQL 数据库连接配置
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    /// 数据库用户名
    pub username: String,
    /// 数据库密码，使用 Secret 包装防止日志泄露
    pub password: Secret<String>,
    /// 数据库端口，支持从字符串反序列化
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// 数据库主机地址
    pub host: String,
    /// 数据库名称
    pub database_name: String,
    /// 是否强制使用 SSL 连接
    pub require_ssl: bool,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}

/// 运行环境枚举
/// 通过 APP_ENVIRONMENT 环境变量设置，默认为 Local
pub enum Environment {
    /// 本地开发环境，加载 configuration/local.yaml
    Local,
    /// 生产环境，加载 configuration/production.yaml
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().trim() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .ssl_mode(ssl_mode)
            .password(self.password.expose_secret())
            .database("postgres")
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}
