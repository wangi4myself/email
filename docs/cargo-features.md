# Cargo.toml 中的 Features 详解

## 什么是 Features

在 Rust 的 `Cargo.toml` 中，`features` 是一种**条件编译机制**，用于启用或禁用 crate（库）中的可选功能。

## 主要作用

1. **减少编译时间和二进制体积** - 只编译你需要的功能
2. **可选依赖** - 某些功能可能需要额外的依赖项
3. **API 扩展** - 启用额外的 API 或实现

## 项目中的实际例子

### Tokio

```toml
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

| Feature | 作用 |
|---------|------|
| `macros` | 启用 `#[tokio::main]` 等宏 |
| `rt-multi-thread` | 启用多线程运行时 |

### Serde

```toml
serde = { version = "1", features = ["derive"] }
```

| Feature | 作用 |
|---------|------|
| `derive` | 启用 `#[derive(Serialize, Deserialize)]` 宏 |

### SQLx

```toml
[dependencies.sqlx]
version = "0.6"
features = ["runtime-actix-rustls", "postgres", "macros", "uuid", "chrono", "migrate"]
```

| Feature | 作用 |
|---------|------|
| `runtime-actix-rustls` | 使用 actix 运行时 + rustls TLS |
| `postgres` | 启用 PostgreSQL 数据库支持 |
| `macros` | 启用 `sqlx::query!` 等编译时检查宏 |
| `uuid` | 支持 UUID 类型 |
| `chrono` | 支持日期时间类型 |
| `migrate` | 启用数据库迁移功能 |

## 语法格式

Features 可以通过两种方式指定：

### 行内格式

```toml
crate_name = { version = "x.x", features = ["feature1", "feature2"] }
```

### 表格格式

```toml
[dependencies.crate_name]
version = "x.x"
features = ["feature1", "feature2"]
```

## 总结

如果不指定这些 features，对应的功能就不会被编译进来，你也无法使用相关的 API。合理使用 features 可以让你的项目更加精简高效。
