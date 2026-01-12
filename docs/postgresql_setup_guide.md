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
