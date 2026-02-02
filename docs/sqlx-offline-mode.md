# SQLx 离线模式指南

## 问题背景

SQLx 的 `query!` 宏在**编译时**会连接数据库验证 SQL 语句。这带来两个问题：

1. CI/CD 环境可能无法访问数据库
2. 本地开发时数据库未运行会导致编译失败

## 解决方案：离线模式

SQLx 提供离线模式，通过预先生成的 `sqlx-data.json` 缓存文件进行编译时验证，无需实际连接数据库。

## 使用步骤

### 1. 生成离线缓存

需要连接到有完整表结构的数据库：

```bash
# 设置数据库连接
export DATABASE_URL="postgresql://user:password@host:port/database?sslmode=require"

# 生成缓存（仅扫描 src/ 目录）
cargo sqlx prepare

# 生成缓存（包含测试文件）
cargo sqlx prepare -- --tests
```

### 2. 启用离线模式

在 `.env` 文件中添加：

```env
SQLX_OFFLINE=true
```

或在构建时设置环境变量：

```bash
SQLX_OFFLINE=true cargo build
```

### 3. 提交缓存文件

将 `sqlx-data.json` 提交到版本控制：

```bash
git add sqlx-data.json
git commit -m "chore: update sqlx offline data"
```

## 常见错误及解决

### 错误 1: failed to find data for query

```
error: failed to find data for query SELECT email, name FROM subscriptions
```

**原因**：查询未被缓存到 `sqlx-data.json`

**解决**：重新生成缓存

```bash
DATABASE_URL="..." cargo sqlx prepare -- --tests
```

### 错误 2: Connection refused

```
error: error communicating with database: Connection refused (os error 61)
```

**原因**：未设置 `DATABASE_URL` 或数据库未运行

**解决**：
- 确保设置了正确的 `DATABASE_URL`
- 或启用离线模式 `SQLX_OFFLINE=true`

### 错误 3: relation "xxx" does not exist

```
error: relation "subscriptions" does not exist
```

**原因**：数据库中没有对应的表

**解决**：先执行迁移

```bash
sqlx migrate run
```

## 工作流程建议

### 本地开发

```bash
# 方式一：运行本地 PostgreSQL
docker run -d --name postgres -e POSTGRES_USER=yadea -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 5432:5432 postgres:16
sqlx migrate run

# 方式二：使用离线模式（.env 中设置 SQLX_OFFLINE=true）
```

### 更新 SQL 查询后

每次修改 `sqlx::query!` 宏中的 SQL 语句后，需要更新缓存：

```bash
DATABASE_URL="..." cargo sqlx prepare -- --tests
git add sqlx-data.json
git commit -m "chore: update sqlx offline data"
```

### CI/CD 构建

Dockerfile 中已配置：

```dockerfile
ENV SQLX_OFFLINE=true
RUN cargo build --release
```

## 注意事项

1. **`--tests` 参数**：`cargo sqlx prepare` 默认只扫描 `src/` 目录。如果测试文件中有 `sqlx::query!`，需要加 `-- --tests`

2. **表结构必须存在**：生成缓存时，数据库必须有完整的表结构（需先执行迁移）

3. **密码安全**：不要将包含真实密码的 `DATABASE_URL` 提交到代码仓库
