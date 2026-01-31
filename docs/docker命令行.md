# Docker 容器常用命令

## 查看容器状态

```bash
# 查看运行中的容器
docker ps

# 查看所有容器（包括已停止的）
docker ps -a
```

## 进入容器

```bash
# 进入运行中的容器（交互式 bash）
docker exec -it <container_id> /bin/bash

# 执行单个命令
docker exec <container_id> <command>
```

## 查看容器内的进程和服务

### 方法 1：在容器内查看

```bash
# 进入容器后执行
ps aux                  # 查看所有进程
ss -tlnp               # 查看监听的端口
netstat -tlnp          # 查看监听的端口（备选）
```

### 方法 2：从宿主机查看

```bash
# 查看容器内运行的进程
docker top <container_id>

# 查看容器详细信息
docker inspect <container_id>

# 查看容器日志
docker logs <container_id>

# 实时查看日志（类似 tail -f）
docker logs -f <container_id>

# 查看最近 100 行日志
docker logs --tail 100 <container_id>
```

## 查看资源使用

```bash
# 实时查看容器 CPU、内存使用
docker stats <container_id>

# 查看所有容器资源使用
docker stats
```

## 容器生命周期管理

```bash
# 停止容器
docker stop <container_id>

# 启动已停止的容器
docker start <container_id>

# 重启容器
docker restart <container_id>

# 删除容器（需先停止）
docker rm <container_id>

# 强制删除运行中的容器
docker rm -f <container_id>
```

## 镜像管理

```bash
# 查看本地镜像
docker images

# 删除镜像
docker rmi <image_id>

# 构建镜像
docker build --tag <name> --file Dockerfile .
```

## docker build 命令详解

```bash
docker build --tag email --file Dockerfile .
```

| 部分 | 含义 |
|------|------|
| `docker build` | Docker 构建镜像的主命令 |
| `--tag email` | 给构建的镜像起名为 `email`（也可以写成 `-t email`） |
| `--file Dockerfile` | 指定使用哪个 Dockerfile（也可以写成 `-f Dockerfile`） |
| `.` | **构建上下文**，指当前目录 |

### 参数说明

#### `--tag email`
- 镜像的名称标签
- 之后可以用 `docker run email` 来运行这个镜像
- 也可以加版本号：`--tag email:1.0`

#### `--file Dockerfile`
- 默认情况下 Docker 会在当前目录找 `Dockerfile`
- 如果文件名就是 `Dockerfile`，这个参数可以省略
- 当使用其他名称（如 `Dockerfile.dev`）时必须指定

#### `.`（构建上下文）
- 告诉 Docker 把**当前目录**的文件发送给 Docker 守护进程
- Dockerfile 中的 `COPY`、`ADD` 命令都是相对于这个上下文路径
- 例如 `COPY ./target/release/email /app` 会从当前目录复制文件

### 简化写法

如果 Dockerfile 就在当前目录且名称为 `Dockerfile`，可以简化为：

```bash
docker build -t email .
```

## 实用技巧

```bash
# 根据镜像名获取容器 ID
docker ps -q --filter ancestor=<image_name>

# 查看特定镜像的容器日志
docker logs $(docker ps -q --filter ancestor=email)

# 清理所有停止的容器
docker container prune

# 清理未使用的镜像
docker image prune
```

## Docker 网络问题：容器无法访问宿主机服务

### 问题描述

在 Docker 容器内，`127.0.0.1` 或 `localhost` 指向的是**容器自身**，而不是宿主机。所以容器内的应用无法通过 `localhost` 连接到宿主机上的数据库或其他服务。

### 解决方案

#### 方法 1：使用 `host.docker.internal`（Mac/Windows 推荐）

Docker Desktop 提供了特殊域名 `host.docker.internal` 来访问宿主机：

```bash
# 运行容器时添加 host 映射
docker run -p 8000:8000 --add-host=host.docker.internal:host-gateway <image_name>
```

然后在应用配置中，将数据库 host 改为 `host.docker.internal`。

#### 方法 2：使用 host 网络模式

```bash
# 容器共享宿主机网络，localhost 可正常工作
docker run --network host <image_name>
```

注意：此模式下 `-p` 端口映射无效，容器直接使用宿主机端口。

#### 方法 3：使用 Docker Compose（最佳实践）

将应用和依赖服务（如数据库）都放在 Docker 网络中：

```yaml
# docker-compose.yaml
version: '3.8'
services:
  db:
    image: postgres:15
    environment:
      POSTGRES_USER: myuser
      POSTGRES_PASSWORD: mypassword
      POSTGRES_DB: mydb
    ports:
      - "5432:5432"

  app:
    build: .
    ports:
      - "8000:8000"
    depends_on:
      - db
    environment:
      DATABASE_HOST: db  # 使用服务名作为主机名
```

启动服务：

```bash
docker-compose up -d
```

### 常见场景

| 场景 | 配置中的 host 值 |
|------|-----------------|
| 本地开发（cargo run） | `localhost` 或 `127.0.0.1` |
| Docker 容器访问宿主机 | `host.docker.internal` |
| Docker Compose 服务间通信 | 服务名（如 `db`） |
