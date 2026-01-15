# Rust lib.rs 运行机制与文件依赖关系

## 概述

`lib.rs` 是 Rust 库的入口文件，它定义了这个 crate（包）对外暴露哪些模块。

## 项目结构

```
src/
├── lib.rs              # 库入口，声明并导出模块
├── main.rs             # 二进制入口，使用 lib.rs 导出的模块
├── configuration.rs    # 配置管理模块
├── startup.rs          # 服务器启动模块
├── telemetry.rs        # 日志追踪模块
└── routes/
    ├── mod.rs          # routes 子模块入口
    ├── health_check.rs # 健康检查路由
    └── subscriptions.rs# 订阅路由
```

## lib.rs 的作用

```rust
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
```

### 关键点：

1. **`pub mod` 做了两件事**：

   - 声明模块存在（告诉编译器去找对应的文件）
   - 公开导出（让外部 crate 可以访问）

2. **模块查找规则**：
   - `pub mod configuration;` → 查找 `src/configuration.rs`
   - `pub mod routes;` → 查找 `src/routes/mod.rs` 或 `src/routes.rs`

## 依赖关系图

```
┌─────────────────────────────────────────────────────┐
│                      main.rs                        │
│   use email::configuration::get_configuration;     │
│   use email::startup::run;                          │
│   use email::telemetry::{get_subscriber, ...};     │
└────────────────────────┬────────────────────────────┘
                         │ 依赖
                         ▼
┌─────────────────────────────────────────────────────┐
│                      lib.rs                         │
│   pub mod configuration;                            │
│   pub mod routes;                                   │
│   pub mod startup;                                  │
│   pub mod telemetry;                                │
└────────────────────────┬────────────────────────────┘
                         │ 导出
     ┌───────────┬───────┴───────┬───────────┐
     ▼           ▼               ▼           ▼
┌─────────┐ ┌─────────┐   ┌──────────┐ ┌───────────┐
│config-  │ │ routes  │   │ startup  │ │ telemetry │
│uration │ │         │   │          │ │           │
└─────────┘ └────┬────┘   └────┬─────┘ └───────────┘
                 │             │
           ┌─────┴─────┐       │ 依赖 routes
           ▼           ▼       │
      ┌─────────┐ ┌─────────┐  │
      │health_  │ │subscri- │◄─┘
      │check    │ │ptions   │
      └─────────┘ └─────────┘
```

## main.rs 如何使用 lib.rs

### Cargo.toml 配置

```toml
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "email"
```

这个配置让：

- `lib.rs` 成为库入口
- `main.rs` 成为可执行文件入口

### main.rs 引用方式

```rust
use email::configuration::get_configuration;
use email::startup::run;
use email::telemetry::{get_subscriber, init_subscriber};
```

- `email` 是 crate 名称（在 Cargo.toml 的 `[package] name` 中定义）
- `main.rs` 通过 crate 名称引用 `lib.rs` 导出的模块

## 模块可见性规则

| 关键字            | 作用                              |
| ----------------- | --------------------------------- |
| `mod xxx;`        | 私有模块，只能在当前 crate 内使用 |
| `pub mod xxx;`    | 公开模块，外部 crate 也可以使用   |
| `pub use xxx::*;` | 重新导出，简化导入路径            |

### routes/mod.rs 示例

```rust
mod health_check;        // 私有声明
mod subscriptions;       // 私有声明

pub use health_check::*; // 重新导出所有公开项
pub use subscriptions::*;
```

这样 `startup.rs` 可以直接写：

```rust
use crate::routes::{health_check, subscribe};
```

而不需要写：

```rust
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
```

## 编译流程

1. **cargo build** 触发编译
2. 编译器从 `lib.rs` 开始解析模块树
3. 递归编译所有 `mod` 声明的模块
4. `main.rs` 作为独立入口，链接到编译好的库

## 为什么要分离 lib.rs 和 main.rs？

1. **代码复用** - `lib.rs` 可以被测试文件和其他 crate 使用
2. **测试隔离** - 集成测试可以像外部用户一样使用库
3. **关注点分离** - `main.rs` 只负责启动，业务逻辑在库中
