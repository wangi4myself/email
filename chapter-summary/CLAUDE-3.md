# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个邮件订阅服务 (Newsletter Subscription Service) 的后端 API，使用 Actix-web 框架和 PostgreSQL 数据库。

## 常用命令

```bash
# 构建项目
cargo build

# 运行服务器
cargo run

# 运行所有测试
cargo test

# 运行单个测试文件
cargo test --test health_check

# 运行单个测试函数
cargo test health_check_works

# 数据库迁移
sqlx migrate run --database-url "postgres://yadea@localhost:5432/newsletter"

# 编译时 SQL 检查（需要数据库连接）
cargo sqlx prepare
```

## 架构

```
src/
├── main.rs           # 入口：读取配置、连接数据库、启动服务器
├── lib.rs            # 库入口，导出公共模块
├── startup.rs        # HTTP 服务器配置和路由注册
├── configuration.rs  # 从 configuration.yaml 加载配置
└── routes/
    ├── mod.rs
    ├── health_check.rs   # GET /health_check
    └── subscriptions.rs  # POST /subscriptions
```

**关键设计:**
- 库/二进制分离：`lib.rs` 导出模块供测试使用，`main.rs` 作为入口
- 数据库连接池 (`PgPool`) 通过 `web::Data` 注入到路由处理器
- 配置文件：`configuration.yaml`（应用配置）和 `.env`（DATABASE_URL）

## 测试

测试位于 `tests/` 目录，使用集成测试模式：
- 每个测试创建随机命名的临时数据库（UUID）实现隔离
- `spawn_app()` 启动测试服务器，绑定随机端口

## 数据库

- 迁移文件：`migrations/`
- 主表：`subscriptions (id, email, name, subscribed_at)`
- `connection_string_without_db()` 连接到 `postgres` 默认数据库用于创建测试数据库
