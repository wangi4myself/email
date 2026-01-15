# Rust Web 应用日志系统详解

本文档介绍 Rust Web 应用中常用的三个日志组件及其协作关系。

## 1. log = "0.4" - 日志门面（Facade）

### 简介
`log` 是 Rust 生态系统中的**标准日志接口**，类似于 Java 中的 SLF4J。它只定义日志接口，不负责具体的输出实现。

### 提供的宏
| 宏 | 级别 | 用途 |
|---|---|---|
| `error!()` | 错误 | 记录错误信息 |
| `warn!()` | 警告 | 记录警告信息 |
| `info!()` | 信息 | 记录一般信息 |
| `debug!()` | 调试 | 记录调试信息 |
| `trace!()` | 追踪 | 记录详细追踪信息 |

### 使用示例
```rust
use log::{info, warn, error};

fn main() {
    info!("应用程序启动");
    warn!("这是一个警告");
    error!("发生了错误: {}", "连接失败");
}
```

---

## 2. env_logger = "0.9" - 日志实现

### 简介
`env_logger` 是 `log` crate 的**具体实现**，负责将日志实际输出到终端，并支持通过环境变量控制日志级别。

### 初始化
```rust
fn main() {
    // 在程序入口处初始化
    env_logger::init();

    // 之后即可使用 log 宏
    log::info!("日志系统已初始化");
}
```

### 环境变量控制
通过 `RUST_LOG` 环境变量控制日志级别：

```bash
# 只显示 info 及以上级别
RUST_LOG=info cargo run

# 显示 debug 及以上级别
RUST_LOG=debug cargo run

# 只显示特定模块的日志
RUST_LOG=my_app=debug cargo run

# 多模块配置
RUST_LOG=my_app=debug,actix_web=info cargo run
```

---

## 3. actix_web::middleware::Logger - HTTP 请求日志中间件

### 简介
这是 Actix-web 框架提供的中间件，用于**自动记录每个 HTTP 请求**的详细信息。

### 记录的信息
- 客户端 IP 地址
- 请求方法（GET、POST 等）
- 请求路径
- HTTP 协议版本
- 响应状态码
- 请求处理耗时

### 使用示例
```rust
use actix_web::{App, HttpServer, web, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())  // 添加日志中间件
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 输出示例
```
INFO  actix_web::middleware::logger > 127.0.0.1 "GET /api/users HTTP/1.1" 200 12ms
INFO  actix_web::middleware::logger > 127.0.0.1 "POST /api/login HTTP/1.1" 401 8ms
```

### 自定义格式
```rust
// 使用自定义格式
Logger::new("%a %r %s %b %T")
```

格式符号说明：
| 符号 | 含义 |
|---|---|
| `%a` | 客户端 IP |
| `%r` | 请求行 |
| `%s` | 状态码 |
| `%b` | 响应体大小 |
| `%T` | 处理时间（秒） |
| `%D` | 处理时间（毫秒） |

---

## 三者协作关系

```
┌─────────────────────────────────────┐
│  actix_web::middleware::Logger      │
│  (生成 HTTP 请求日志)                │
└──────────────┬──────────────────────┘
               │ 调用
               ▼
┌─────────────────────────────────────┐
│           log crate                 │
│  (提供统一的日志接口和宏)            │
└──────────────┬──────────────────────┘
               │ 委托
               ▼
┌─────────────────────────────────────┐
│         env_logger                  │
│  (实际输出到终端)                    │
└─────────────────────────────────────┘
```

### 总结
- **log**：定义"怎么记录"（接口）
- **env_logger**：决定"输出到哪里"（实现）
- **Logger 中间件**：自动记录 HTTP 请求（应用层）

---

## 完整示例

```rust
use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use log::info;

async fn health_check() -> HttpResponse {
    info!("健康检查接口被调用");
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化 env_logger
    env_logger::init();

    info!("服务器正在启动...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

运行命令：
```bash
RUST_LOG=info cargo run
```
