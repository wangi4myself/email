# 第四章代码变更总结

**提交范围**: `3bb3cb1` (第三章完结) → `12a9587` (第四章完结)

---

## 一、新增文件

### 1. `src/telemetry.rs` (核心新增)

新建日志/追踪模块，提供：

```rust
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_subscriber::fmt::MakeWriter;

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
```

### 2. 文档文件 (docs/)

| 文件                            | 内容                          |
| ------------------------------- | ----------------------------- |
| `rust-double-colon-vs-dot.md`   | Rust `::` 与 `.` 操作符区别   |
| `rust-impl-trait-explanation.md`| `impl Trait` 返回类型详解     |
| `rust-lib-rs-explanation.md`    | lib.rs 模块系统说明           |
| `rust-logging-system.md`        | Rust Web 应用日志系统         |
| `toml-table-syntax.md`          | TOML 表格语法                 |

---

## 二、依赖更新 (Cargo.toml)

### 新增依赖

```toml
tracing = {version = "0.1", features = ["log"]}
tracing-subscriber = {version = "0.3", features = ["env-filter","registry"]}
tracing-bunyan-formatter = "0.3"
tracing-log = "0.1"
secrecy = {version = "0.8", features = ["serde"]}
tracing-actix-web = "0.6"

[dev-dependencies]
once_cell = "1"
```

**用途**:

| 依赖                    | 作用                                |
| ----------------------- | ----------------------------------- |
| `tracing`               | 结构化日志框架                      |
| `tracing-subscriber`    | 日志订阅者配置                      |
| `tracing-bunyan-formatter` | Bunyan JSON 格式化输出           |
| `tracing-log`           | 将 log crate 日志桥接到 tracing    |
| `secrecy`               | 敏感信息（密码）保护                |
| `tracing-actix-web`     | HTTP 请求追踪中间件                 |
| `once_cell`             | 测试中单次初始化日志                |

---

## 三、配置模块变更 (src/configuration.rs)

### 密码类型改为 Secret

```rust
// 之前
pub password: String,
pub fn connection_string(&self) -> String { ... }

// 之后
use secrecy::{ExposeSecret, Secret};

pub password: Secret<String>,
pub fn connection_string(&self) -> Secret<String> {
    Secret::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        self.username,
        self.password.expose_secret(),  // 显式暴露
        self.host,
        self.port,
        self.database_name
    ))
}
```

**目的**: 防止密码意外泄露到日志中。访问密码必须显式调用 `.expose_secret()`。

---

## 四、启动模块变更 (src/startup.rs)

### 添加请求追踪中间件

```rust
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())  // 新增：自动记录每个 HTTP 请求
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
```

---

## 五、路由模块重构 (src/routes/subscriptions.rs)

### 主要变更

#### 1. 添加 tracing instrument 宏

```rust
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(subscribe_email = %form.email, subscribe_name = %form.name)
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to add new subscriber: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
```

#### 2. 抽取数据库操作为独立函数

```rust
#[tracing::instrument(name = "Saving new subscriber details in the database", skip(pool, form))]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query:{:?}", e);
        e
    })?;
    Ok(())
}
```

#### 3. 日志从 println! 改为 tracing::error!

```rust
// 之前
println!("Failed to execute query:{}", e);

// 之后
tracing::error!("Failed to execute query:{:?}", e);
```

---

## 六、主程序变更 (src/main.rs)

### 添加日志初始化

```rust
use email::configuration::get_configuration;
use email::startup::run;
use email::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 新增：初始化日志系统
    let subscriber = get_subscriber("email".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // 使用 expose_secret() 访问密码
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to the database.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Listening on port {}", configuration.application_port);
    run(listener, connection_pool)?.await
}
```

---

## 七、测试模块变更 (tests/health_check.rs)

### 1. 添加全局日志初始化（Lazy）

```rust
use once_cell::sync::Lazy;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        // 输出到 stdout
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        // 输出到 sink（丢弃）
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});
```

**用途**:

- 默认测试时日志静默
- 设置 `TEST_LOG=1` 环境变量可查看日志

### 2. 简化测试代码

```rust
// 之前：重新创建数据库连接
let configuration = get_configuration()...;
let mut connection = PgConnection::connect(&connection_string)...;

// 之后：直接使用 TestApp 的连接池
let app = spawn_app().await;
sqlx::query!(...).fetch_one(&app.db_pool)...
```

---

## 八、lib.rs 清理

移除多余空行，添加 telemetry 模块导出：

```rust
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;  // 新增
```

---

## 总结

| 类别       | 变更内容                                                       |
| ---------- | -------------------------------------------------------------- |
| 日志系统   | 引入 tracing + bunyan formatter，结构化 JSON 日志              |
| 安全性     | 使用 secrecy 保护密码，防止意外泄露                            |
| 可观测性   | TracingLogger 中间件自动追踪 HTTP 请求                         |
| 代码质量   | 抽取 insert_subscriber 函数，添加 instrument 宏                |
| 测试改进   | Lazy 初始化日志，简化测试代码，支持 TEST_LOG 环境变量          |

---

## 运行日志示例

使用 bunyan 格式化输出：

```bash
cargo run | bunyan
```

输出示例：

```
[2026-01-15T07:18:43.484Z]  INFO: email/31582: [HTTP REQUEST - START]
    http.method=POST
    http.route=/subscriptions
    request_id=2f50c1c3-0781-4571-a5f8-27caa6bb11ec

[2026-01-15T07:18:43.484Z]  INFO: email/31582: [ADDING A NEW SUBSCRIBER - START]
    subscribe_email=zhangsan@example.com
    subscribe_name=张三

[2026-01-15T07:18:43.487Z]  INFO: email/31582: [HTTP REQUEST - END]
    http.status_code=200
    elapsed_milliseconds=3
```

测试时查看日志：

```bash
TEST_LOG=1 cargo test --test health_check | bunyan
```
