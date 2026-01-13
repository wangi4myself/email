# macOS PostgreSQL 安装指南

## 背景

在 macOS 上运行 `init_db.sh` 脚本时遇到 Docker 连接错误：

```
failed to connect to the docker API at unix:///Users/yadea/.docker/run/docker.sock
```

原因：`brew install docker` 只安装了 Docker CLI 工具，而非 Docker 守护进程（daemon）。

---

## 方案对比

### 方案一：Docker Desktop（官方方案）

**优点：**
- 官方支持，功能完整
- 图形界面，易于管理容器
- 包含 Kubernetes 支持

**缺点：**
- 体积大（约 2GB）
- 资源占用较多
- 商业使用可能需要付费许可

**安装命令：**
```bash
brew install --cask docker
```

---

### 方案二：Colima（轻量级替代）

**优点：**
- 轻量级，资源占用少
- 完全免费开源
- 与 Docker CLI 完全兼容

**缺点：**
- 无图形界面
- 需要手动启动

**安装命令：**
```bash
brew install colima
colima start
```

---

### 方案三：本地安装 PostgreSQL（不用 Docker）

**优点：**
- 最轻量，无需虚拟化
- 启动快，性能好

**缺点：**
- 需要手动管理数据库
- 需要修改 init_db.sh 脚本或单独配置

**安装命令：**
```bash
brew install postgresql@16
brew services start postgresql@16
```

---

### 方案四：OrbStack（付费/免费个人版）

**优点：**
- 性能优秀，启动快
- 资源占用比 Docker Desktop 少
- 有图形界面

**缺点：**
- 商业使用需付费

**安装命令：**
```bash
brew install --cask orbstack
```

---

## 推荐方案

| 使用场景 | 推荐方案 |
|---------|---------|
| 个人学习/开发 | **Colima** - 轻量免费 |
| 需要图形界面 | **Docker Desktop** |
| 追求性能 | **OrbStack** 或 **本地 PostgreSQL** |

---

## 本地 PostgreSQL 安装步骤（已选方案）

### 步骤 1：安装 PostgreSQL

```bash
brew install postgresql@16
```

### 步骤 2：启动服务

```bash
brew services start postgresql@16
```

### 步骤 3：配置 PATH

```bash
echo 'export PATH="/opt/homebrew/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 步骤 4：创建项目数据库

```bash
createdb newsletter
```

### 步骤 5：验证连接

```bash
psql -d newsletter -c "SELECT 1"
```

---

## 安装结果

| 步骤 | 状态 |
|-----|------|
| 安装 PostgreSQL 16 | 完成 |
| 启动服务 | 完成 |
| 配置 PATH | 完成 |
| 创建数据库 newsletter | 完成 |
| 验证连接 | 完成 |

---

## 连接信息

| 配置项 | 值 |
|-------|-----|
| 主机 | localhost |
| 端口 | 5432 |
| 数据库 | newsletter |
| 用户 | yadea (macOS 用户名) |
| 密码 | 无需密码 |

### 连接字符串

```
postgres://yadea@localhost:5432/newsletter
```

---

## 常用命令

```bash
# 查看服务状态
brew services info postgresql@16

# 停止服务
brew services stop postgresql@16

# 启动服务
brew services start postgresql@16

# 重启服务
brew services restart postgresql@16

# 连接数据库
psql -d newsletter

# 列出所有数据库
psql -l

# 删除数据库
dropdb newsletter

# 创建数据库
createdb newsletter
```

---

## 在 Rust 项目中使用

### 环境变量配置

创建 `.env` 文件：

```env
DATABASE_URL=postgres://yadea@localhost:5432/newsletter
```

### Cargo.toml 添加依赖

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
```

---

## 备注

- 使用本地 PostgreSQL 后，无需再运行 `init_db.sh` 脚本
- PostgreSQL 会在系统启动时自动启动（通过 brew services）
- 数据存储位置：`/opt/homebrew/var/postgresql@16`

---

## 常见问题 (FAQ)

### Q1: 执行 `sqlx migrate run` 时提示缺少 `--database-url` 参数

**错误信息：**
```
error: The following required arguments were not provided:
    --database-url <DATABASE_URL>
```

**原因：**
`sqlx migrate run` 需要知道数据库连接地址。

**解决方案：**

**方式 1: 使用命令行参数**
```bash
sqlx migrate run --database-url "postgres://yadea@localhost:5432/newsletter"
```

**方式 2: 设置环境变量**
```bash
export DATABASE_URL="postgres://yadea@localhost:5432/newsletter"
sqlx migrate run
```

**方式 3: 创建 `.env` 文件（推荐）**

在项目根目录创建 `.env` 文件：
```env
DATABASE_URL=postgres://yadea@localhost:5432/newsletter
```

然后直接运行：
```bash
sqlx migrate run
```

---

### Q2: `sqlx migrate run` 执行成功后提示 "Applied xxx/migrate create subscriptions table"，这是什么意思？

**含义：**
这表示数据库迁移执行成功，已在数据库中创建了 `subscriptions` 表。

**迁移机制说明：**

1. **版本控制** - 文件名中的时间戳（如 `20260112013036`）用于标识迁移的执行顺序
2. **幂等性** - sqlx 会在数据库中记录已执行的迁移，重复运行不会重复执行
3. **表结构** - 本次迁移创建的 `subscriptions` 表结构：

| 字段 | 类型 | 约束 |
|------|------|------|
| `id` | uuid | 主键，非空 |
| `email` | TEXT | 非空，唯一 |
| `name` | TEXT | 非空 |
| `subscribed_at` | timestamptz | 非空（带时区的时间戳） |

**验证表是否创建成功：**
```bash
psql -d newsletter -c "\d subscriptions"
```

---

### Q3: 如何在 DBeaver 中查看创建的表？

**步骤 1: 创建数据库连接（如果还没有）**

- 点击左上角 **"新建数据库连接"** 按钮（或 `Cmd + Shift + N`）
- 选择 **PostgreSQL**
- 填写连接信息：
  - Host: `localhost`
  - Port: `5432`
  - Database: `newsletter`
  - Username: `yadea`
  - Password: 留空
- 点击 **"测试连接"**，成功后点击 **"完成"**

**步骤 2: 查看 subscriptions 表**

在左侧导航展开：
```
newsletter
  └── Schemas
       └── public
            └── Tables
                 └── subscriptions
```

**步骤 3: 查看表结构**

- 双击 `subscriptions` 表
- 选择 **"Properties"** 或 **"Columns"** 标签页查看字段定义

**步骤 4: 查看表数据**

- 双击表后选择 **"Data"** 标签页
- 或者右键表 → **"查看数据"** → **"所有行"**

---

### Q4: 为什么应该使用 PgPool 而不是 PgConnection？

**PgConnection vs PgPool 的区别：**

| 特性 | PgConnection | PgPool |
|------|--------------|--------|
| 连接数 | 单个连接 | 多个连接的池 |
| Clone | 不支持 | 支持 |
| 并发 | 一次只能处理一个查询 | 可同时处理多个查询 |
| 适用场景 | 脚本、测试、一次性任务 | Web 服务、高并发应用 |

**为什么 Web 服务需要 PgPool：**

1. **HttpServer 需要 Clone**
   ```rust
   HttpServer::new(move || { ... })
   ```
   Actix-web 会启动多个 worker 线程，每个都需要一份应用状态。`PgConnection` 不能 Clone，而 `PgPool` 可以。

2. **并发请求处理**
   - 单个 `PgConnection`：请求 A 查询时，请求 B 必须等待
   - `PgPool`：请求 A 和 B 可以从池中获取不同连接，同时查询

3. **连接复用**
   - 创建数据库连接是昂贵操作（TCP握手、认证等）
   - 连接池预先创建连接，请求直接复用，性能更高

4. **自动管理**
   - 连接断开自动重连
   - 空闲连接自动回收
   - 防止连接泄漏

**简单类比：**

- `PgConnection` = 一部电话，一次只能一个人打
- `PgPool` = 呼叫中心，有多条线路，多人可以同时打电话

**代码示例：**

```rust
use sqlx::PgPool;

// 创建连接池
let pool = PgPool::connect(&database_url).await?;

// 在 handler 中使用
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    sqlx::query!(...)
        .execute(pool.get_ref())
        .await;
    // ...
}
```

---

### Q5: 测试时报错 `database "yadea" does not exist`

**错误信息：**
```
Failed to connect to Postgres: Database(PgDatabaseError {
    severity: Fatal,
    code: "3D000",
    message: "database \"yadea\" does not exist"
})
```

**原因：**

测试代码中使用 `connection_string_without_db()` 连接 PostgreSQL 来创建临时测试数据库。当连接字符串不指定数据库名时，PostgreSQL 默认会尝试连接到与用户名同名的数据库。

例如，配置中用户名是 `yadea`，PostgreSQL 就会尝试连接 `yadea` 数据库，但这个数据库不存在。

**连接字符串示例：**
```
# 不指定数据库 - PostgreSQL 默认连接用户名同名的数据库
postgres://yadea:password@127.0.0.1:5432

# 指定数据库
postgres://yadea:password@127.0.0.1:5432/newsletter
```

**解决方案：**

修改 `src/configuration.rs` 中的 `connection_string_without_db()` 方法，显式连接到 `postgres` 数据库（PostgreSQL 安装时默认创建的数据库）：

```rust
// 修改前
pub fn connection_string_without_db(&self) -> String {
    format!(
        "postgres://{}:{}@{}:{}",
        self.username, self.password, self.host, self.port
    )
}

// 修改后
pub fn connection_string_without_db(&self) -> String {
    format!(
        "postgres://{}:{}@{}:{}/postgres",
        self.username, self.password, self.host, self.port
    )
}
```

**为什么用 `postgres` 数据库：**

1. `postgres` 是 PostgreSQL 安装时自动创建的系统数据库
2. 它在任何 PostgreSQL 实例中都存在
3. 通常用于管理操作，如创建/删除其他数据库

**验证修复：**
```bash
cargo test --test health_check
```

所有测试应该通过。
