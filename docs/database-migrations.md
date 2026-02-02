# 数据库迁移指南

## 什么是迁移

迁移（Migration）是一种版本化管理数据库结构的方式。每个迁移文件包含 SQL 语句，用于创建、修改或删除数据库表结构。

## 迁移文件位置

```
migrations/
└── 20260112013036_create_subscriptions_table.sql
```

文件名格式：`{时间戳}_{描述}.sql`

## 迁移文件内容

`20260112013036_create_subscriptions_table.sql`:

```sql
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
```

创建 `subscriptions` 表：

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | UUID | 主键，唯一标识 |
| `email` | TEXT | 邮箱地址（唯一约束） |
| `name` | TEXT | 订阅者姓名 |
| `subscribed_at` | timestamptz | 订阅时间（带时区） |

## 执行时机

迁移**不会自动执行**，需要手动触发。

### 本地开发

```bash
# 确保 .env 中有 DATABASE_URL
sqlx migrate run
```

### 生产环境（DigitalOcean）

**方式一：命令行执行**

```bash
DATABASE_URL="postgresql://user:pass@host:port/db?sslmode=require" sqlx migrate run
```

**方式二：DBeaver 直接执行 SQL**

连接到数据库后，直接执行迁移文件中的 SQL 语句。

**方式三：应用启动时自动执行（推荐）**

在 `src/main.rs` 中添加：

```rust
// 连接数据库
let connection_pool = PgPoolOptions::new()
    .acquire_timeout(std::time::Duration::from_secs(2))
    .connect_with(configuration.database.with_db())
    .await
    .expect("Failed to connect to database");

// 执行迁移
sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database");
```

## SQLx 迁移特性

- **幂等性**：已执行的迁移不会重复执行
- **版本追踪**：SQLx 在数据库中创建 `_sqlx_migrations` 表记录已执行的迁移
- **顺序执行**：按文件名时间戳顺序执行

## 常用命令

```bash
# 执行所有待执行的迁移
sqlx migrate run

# 创建新的迁移文件
sqlx migrate add <name>

# 回滚上一次迁移（需要有对应的 down 文件）
sqlx migrate revert

# 查看迁移状态
sqlx migrate info
```

## 离线模式

当无法连接数据库时（如 CI 构建），使用离线模式：

```bash
# 生成离线缓存（需要数据库连接）
cargo sqlx prepare

# 构建时使用缓存
SQLX_OFFLINE=true cargo build
```

这会生成 `sqlx-data.json` 文件，包含所有 SQL 查询的元数据。
