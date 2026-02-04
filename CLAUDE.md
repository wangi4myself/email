# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 学习者背景

本项目是一个 Rust 系统学习项目。项目维护者是一名熟悉 **Node.js / TypeScript** 的开发者。

**Claude 指导原则：**
- 当解释 Rust 概念时，请尽量**类比 TypeScript** 进行说明
- 对比 Rust 和 TypeScript 的相似与不同之处
- 使用开发者熟悉的 Node.js 生态概念来建立理解桥梁

## 常用命令

### 构建与运行

```bash
cargo build                    # 构建项目
cargo run                      # 运行应用 (默认端口 8000)
```

### 测试

```bash
cargo test                     # 运行所有测试
cargo test <test_name>         # 运行单个测试
TEST_LOG=true cargo test       # 运行测试并输出日志
```

### 数据库

```bash
sqlx migrate run               # 执行数据库迁移
sqlx migrate add <name>        # 创建新的迁移文件
```

### 离线模式 (用于 CI 或无数据库环境编译)

```bash
cargo sqlx prepare             # 生成离线查询数据
```

## 项目架构

这是一个基于 **Actix-web** 和 **SQLx** 的邮件订阅服务，采用分层架构：

```
src/
├── main.rs          # 入口：初始化 telemetry、数据库连接池、启动服务器
├── lib.rs           # 库入口，导出所有模块
├── startup.rs       # HTTP 服务器配置，路由注册
├── configuration.rs # 配置管理 (多环境支持)
├── telemetry.rs     # 日志/追踪设置 (tracing + bunyan)
└── routes/          # HTTP 路由处理器
    ├── health_check.rs
    └── subscriptions.rs
```

### 配置系统

- 配置文件位于 `configuration/` 目录
- 使用 `base.yaml` + 环境特定文件 (`local.yaml`, `production.yaml`) 合并
- 通过 `APP_ENVIRONMENT` 环境变量切换环境 (默认 `local`)

### 数据库

- PostgreSQL，通过 SQLx 连接池管理
- 迁移文件在 `migrations/` 目录
- 使用 `sqlx::query!` 宏进行编译时 SQL 验证

### 测试策略

- 集成测试位于 `tests/health_check.rs`
- 每个测试创建独立的临时数据库 (UUID 命名)
- 使用 `TEST_LOG=true` 启用测试日志输出

## 关键依赖

| 依赖      | 用途                   |
| --------- | ---------------------- |
| actix-web | Web 框架               |
| sqlx      | 异步 PostgreSQL 客户端 |
| tracing   | 结构化日志             |
| secrecy   | 敏感数据保护           |
| config    | 配置文件解析           |
