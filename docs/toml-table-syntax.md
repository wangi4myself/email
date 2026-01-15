# TOML 表格语法详解

## 背景

在 `Cargo.toml` 中，你可能会看到这种写法：

```toml
[dependencies.sqlx]
version = "0.6"
features = ["runtime-actix-rustls", "postgres", "macros", "uuid", "chrono", "migrate"]
```

这是 TOML 的 **表格（table）语法**，与内联写法完全等价。

## 两种写法对比

### 写法 1：内联格式（一行）

```toml
sqlx = { version = "0.6", features = ["runtime-actix-rustls", "postgres", "macros", "uuid", "chrono", "migrate"] }
```

### 写法 2：表格格式（多行）

```toml
[dependencies.sqlx]
version = "0.6"
features = ["runtime-actix-rustls", "postgres", "macros", "uuid", "chrono", "migrate"]
```

**两者完全等价**，Cargo 解析结果一样。

## 语法原理

```toml
[dependencies.sqlx]
```

这是 TOML 的 **点号路径** 语法，表示嵌套结构：

```
dependencies
    └── sqlx
          ├── version = "0.6"
          └── features = [...]
```

等同于 JSON：

```json
{
  "dependencies": {
    "sqlx": {
      "version": "0.6",
      "features": ["runtime-actix-rustls", "postgres", ...]
    }
  }
}
```

## 为什么选择表格格式？

| 情况           | 推荐格式                                        |
| -------------- | ----------------------------------------------- |
| 简单依赖       | `name = "1.0"`                                  |
| 少量 features  | `name = { version = "1.0", features = ["a"] }` |
| **多个 features** | `[dependencies.name]` 表格格式               |
| 需要添加注释   | 表格格式                                        |

## 混合用法示例

```toml
[dependencies]
# 简单依赖 - 直接写版本号
actix-web = "4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

# 复杂依赖 - 使用表格格式
[dependencies.sqlx]
version = "0.6"
features = [
    "runtime-actix-rustls",
    "postgres",
    "macros",
    "uuid",
    "chrono",
    "migrate"
]

[dependencies.serde]
version = "1"
features = ["derive"]
```

## 其他常见的表格写法

### dev-dependencies

```toml
[dev-dependencies.reqwest]
version = "0.11"
features = ["json", "blocking"]
```

### build-dependencies

```toml
[build-dependencies.cc]
version = "1.0"
```

### 带 target 平台限定

```toml
[target.'cfg(windows)'.dependencies.winapi]
version = "0.3"
features = ["winuser"]
```

## 总结

- `[dependencies.xxx]` 是表格语法，用于复杂依赖配置
- 与内联 `{ version, features }` 完全等价
- features 多时用表格格式更清晰
- 可以自由混合使用两种格式
