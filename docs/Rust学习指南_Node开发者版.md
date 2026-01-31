# Rust 学习指南 - Node.js 开发者版

## 📌 前言

作为一名有 Node.js 经验的 Web 开发者,你已经具备了编程基础、异步编程思维和 Web 开发经验。Rust 将为你打开一扇通往系统级编程、极致性能和内存安全的大门。

### 为什么 Web 开发者要学 Rust?

- **性能提升**: Rust 的性能接近 C/C++,可将计算密集型任务提速 10-100 倍
- **WebAssembly**: 在浏览器中运行高性能代码的最佳选择
- **全栈能力**: 后端服务、CLI 工具、系统编程全覆盖
- **职业发展**: Rust 连续多年成为最受喜爱的编程语言(Stack Overflow 调查)
- **现代化工具链**: Cargo 的体验不输 npm,甚至更优秀

---

## 🎯 学习路径规划

### 阶段一: 基础入门 (2-4 周)

**目标**: 理解 Rust 核心概念,建立与 JavaScript/Node.js 的对比思维

#### 核心概念对比

| 概念 | Node.js/JavaScript | Rust |
|------|-------------------|------|
| **内存管理** | 垃圾回收(GC) | 所有权系统(Ownership) |
| **类型系统** | 动态类型 + TypeScript可选 | 静态强类型 + 类型推导 |
| **并发模型** | 事件循环 + 单线程 | 多线程 + async/await |
| **错误处理** | try/catch + Promise rejection | Result<T, E> + Option<T> |
| **包管理** | npm/yarn/pnpm | Cargo |
| **运行环境** | V8 引擎 | 直接编译为机器码 |

#### 必学核心概念

1. **所有权系统 (Ownership)** - Rust 最独特的特性
   ```rust
   // JavaScript: 自动垃圾回收
   let data = { name: "test" };
   let copy = data; // 引用复制

   // Rust: 所有权转移
   let data = String::from("test");
   let moved = data; // 所有权转移,data 不再可用
   ```

2. **借用和引用 (Borrowing)**
   ```rust
   // 不可变借用
   let s = String::from("hello");
   let len = calculate_length(&s); // &s 是借用

   // 可变借用
   let mut s = String::from("hello");
   change(&mut s);
   ```

3. **生命周期 (Lifetimes)** - 编译器确保引用有效性
4. **错误处理** - Result 和 Option
   ```rust
   // JavaScript
   try {
     const data = await fetch(url);
   } catch (error) {
     console.error(error);
   }

   // Rust
   match fetch_data() {
     Ok(data) => println!("Success: {:?}", data),
     Err(e) => eprintln!("Error: {}", e),
   }
   ```

### 阶段二: Web 开发实战 (4-8 周)

**目标**: 使用 Rust 构建 Web 应用,熟悉生态系统

#### Rust Web 框架对比

| 框架 | 类似 Node.js 框架 | 特点 | 学习难度 |
|------|------------------|------|---------|
| **Axum** | Express.js | 现代化、基于 Tokio、类型安全 | ⭐⭐⭐ |
| **Actix-web** | Fastify | 高性能、Actor 模型 | ⭐⭐⭐⭐ |
| **Rocket** | NestJS | 全功能、易用、宏魔法 | ⭐⭐ |
| **Warp** | Koa | 函数式、组合式 | ⭐⭐⭐⭐ |

**推荐起点**: Axum (2024 年最流行的选择)

#### 示例: Express.js vs Axum

```javascript
// Express.js
const express = require('express');
const app = express();

app.get('/users/:id', async (req, res) => {
  const user = await getUserById(req.params.id);
  res.json(user);
});

app.listen(3000);
```

```rust
// Axum
use axum::{Router, extract::Path, Json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    let user = get_user_by_id(id).await;
    Json(user)
}
```

### 阶段三: WebAssembly + 前端集成 (2-4 周)

**目标**: 用 Rust 编写高性能前端模块

#### 技术栈

- **wasm-pack**: 构建 Rust → WASM 的工具链
- **wasm-bindgen**: Rust 与 JavaScript 互操作
- **Yew/Leptos**: Rust 前端框架(类似 React)

#### 使用场景

- 图像/视频处理
- 加密算法
- 数据可视化
- 游戏引擎
- 科学计算

```rust
// Rust 代码
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> Vec<u8> {
    // 高性能图像处理
    data.iter().map(|&x| x.saturating_add(10)).collect()
}
```

```javascript
// JavaScript 中使用
import init, { process_image } from './pkg/image_processor.js';

await init();
const processed = process_image(imageData);
```

---

## 📚 精选学习资源

### 🏆 官方资源 (必读)

1. **The Rust Programming Language** (The Book)
   - 链接: https://doc.rust-lang.org/book/
   - 中文版: https://kaisery.github.io/trpl-zh-cn/
   - **必读章节**: 1-10章(基础)、13-16章(进阶)
   - 评价: ⭐⭐⭐⭐⭐ 最权威的入门教程

2. **Rust by Example**
   - 链接: https://doc.rust-lang.org/rust-by-example/
   - 特点: 代码示例驱动学习
   - 评价: ⭐⭐⭐⭐⭐ 实战练习最佳

3. **Rustlings** (交互式练习)
   - 链接: https://github.com/rust-lang/rustlings
   - 安装: `curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash`
   - 评价: ⭐⭐⭐⭐⭐ 边学边练,强烈推荐

### 🎓 在线课程

#### 付费课程 (高质量)

1. **Ultimate Rust Crash Course** - Udemy
   - 讲师: Nathan Stocks
   - 时长: 8 小时
   - 价格: $15-20 (促销时)
   - 适合: 完全初学者
   - 评分: 4.6/5.0

2. **Rust Programming: The Complete Developer's Guide** - Udemy
   - 讲师: Stephen Grider
   - 时长: 20+ 小时
   - 特点: 项目驱动,包含 Web 开发
   - 评分: 4.7/5.0

3. **The Rust Programming Language** - Frontend Masters
   - 讲师: Richard Feldman
   - 时长: 5 小时
   - 特点: 高级话题,适合有基础的开发者
   - 评分: ⭐⭐⭐⭐⭐

4. **Rust Essential Training** - LinkedIn Learning
   - 时长: 3.5 小时
   - 特点: 企业级培训标准
   - 免费试用: 1 个月

#### 免费课程

1. **Microsoft: Take your first steps with Rust**
   - 平台: Microsoft Learn
   - 链接: https://learn.microsoft.com/en-us/training/paths/rust-first-steps/
   - 评价: ⭐⭐⭐⭐ 结构化、有项目

2. **Comprehensive Rust** - Google Android 团队
   - 链接: https://google.github.io/comprehensive-rust/
   - 时长: 4 天课程
   - 评价: ⭐⭐⭐⭐⭐ 2024 年最佳免费课程

3. **Exercism - Rust Track**
   - 链接: https://exercism.org/tracks/rust
   - 特点: 163+ 练习题 + 导师指导
   - 评价: ⭐⭐⭐⭐⭐ 免费且有导师反馈

### 📖 推荐书籍

1. **《Programming Rust》** (O'Reilly, 2nd Edition 2021)
   - 作者: Jim Blandy, Jason Orendorff
   - 难度: 中级
   - 评价: ⭐⭐⭐⭐⭐ 最全面的 Rust 书籍

2. **《Rust in Action》** (Manning, 2021)
   - 作者: Tim McNamara
   - 特点: 系统编程实战
   - 适合: 有其他语言经验的开发者

3. **《Zero to Production in Rust》** (2022)
   - 作者: Luca Palmieri
   - 特点: 完整的 Web 后端项目
   - 评价: ⭐⭐⭐⭐⭐ Web 开发者必读
   - 链接: https://www.zero2prod.com/

4. **《Rust for Rustaceans》** (No Starch Press, 2021)
   - 作者: Jon Gjengset
   - 难度: 高级
   - 适合: 进阶学习

### 🎬 YouTube 频道

1. **Let's Get Rusty**
   - 链接: https://www.youtube.com/@letsgetrusty
   - 特点: The Book 视频版 + 项目实战
   - 评价: ⭐⭐⭐⭐⭐

2. **Jon Gjengset**
   - 链接: https://www.youtube.com/@jonhoo
   - 特点: 深度技术解析、直播编码
   - 难度: 中高级

3. **No Boilerplate**
   - 短视频形式,快速了解 Rust 特性
   - 链接: https://www.youtube.com/@NoBoilerplate

---

## 🛠️ 实战项目推荐

### 初级项目 (入门阶段)

1. **CLI 工具系列**
   ```bash
   # 替代常用 Node.js CLI 工具
   - 文件搜索工具 (替代 find)
   - JSON 格式化工具
   - HTTP 客户端 (类似 axios CLI)
   - 代码统计工具 (类似 cloc)
   ```

2. **REST API 服务**
   - 技术栈: Axum + SQLx + PostgreSQL
   - 功能: CRUD 操作、JWT 认证
   - 参考: https://github.com/tokio-rs/axum/tree/main/examples

3. **WebAssembly 计算器**
   - 技术栈: Rust + wasm-pack + Vanilla JS
   - 学习: WASM 基础、JS 互操作

### 中级项目 (进阶阶段)

1. **实时聊天应用**
   - 后端: Axum + WebSocket + Redis
   - 前端: React + WASM
   - 学习: 异步编程、并发控制

2. **图像处理 Web 服务**
   - 技术栈: Actix-web + image crate
   - 功能: 图片裁剪、滤镜、格式转换
   - 学习: 性能优化、流式处理

3. **Markdown 转 PDF 服务**
   - 技术栈: Rocket + headless_chrome
   - 对比: Node.js puppeteer 的 Rust 替代

### 高级项目 (精通阶段)

1. **分布式任务队列**
   - 技术栈: Tokio + Redis + gRPC
   - 类似: Bull.js 的 Rust 实现
   - 学习: 分布式系统、消息队列

2. **GraphQL 服务器**
   - 技术栈: Async-graphql + Axum
   - 对比: Apollo Server 的性能对比

3. **自定义 ORM**
   - 学习 SQLx、Diesel 的设计思路
   - 理解宏系统、trait 系统

### 开源项目学习

**推荐阅读源码的项目** (由简到难):

1. **ripgrep** - 代码搜索工具
   - GitHub: https://github.com/BurntSushi/ripgrep
   - 学习: CLI、正则表达式、性能优化

2. **tokei** - 代码统计工具
   - GitHub: https://github.com/XAMPPRocky/tokei
   - 学习: 文件处理、并行计算

3. **starship** - Shell 提示符
   - GitHub: https://github.com/starship/starship
   - 学习: 跨平台、Git 集成

4. **deno** - JavaScript/TypeScript 运行时
   - GitHub: https://github.com/denoland/deno
   - 学习: V8 绑定、系统编程

---

## 🌐 Rust Web 生态系统

### 后端开发

#### HTTP 框架

```rust
// 1. Axum - 推荐用于新项目
use axum::{Router, routing::get};

let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }));

// 2. Actix-web - 性能极致
use actix_web::{web, App, HttpServer};

HttpServer::new(|| {
    App::new()
        .route("/", web::get().to(|| async { "Hello!" }))
})

// 3. Rocket - 易用性优先
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
```

#### 数据库

| Crate | 类似 Node.js 库 | 特点 |
|-------|----------------|------|
| **SQLx** | node-postgres | 编译时 SQL 检查 |
| **Diesel** | TypeORM | 强类型 ORM |
| **SeaORM** | Prisma | 异步 ORM,易用 |
| **mongodb** | mongodb | 官方驱动 |
| **redis** | ioredis | 异步 Redis 客户端 |

#### 推荐组合

```toml
# Cargo.toml - 现代化 Web 后端
[dependencies]
axum = "0.7"           # Web 框架
tokio = { version = "1", features = ["full"] }  # 异步运行时
sqlx = "0.7"           # 数据库
serde = "1.0"          # JSON 序列化
tower = "0.4"          # 中间件
tracing = "0.1"        # 日志追踪
```

### WebAssembly 开发

#### 工具链

1. **wasm-pack** - 构建和发布
   ```bash
   cargo install wasm-pack
   wasm-pack build --target web
   ```

2. **trunk** - WASM Web 应用打包器
   ```bash
   cargo install trunk
   trunk serve  # 类似 webpack-dev-server
   ```

#### 前端框架

| 框架 | 类似 | 特点 | 成熟度 |
|------|------|------|--------|
| **Yew** | React | 组件化、虚拟 DOM | ⭐⭐⭐⭐ |
| **Leptos** | SolidJS | 细粒度响应式 | ⭐⭐⭐ (新兴) |
| **Dioxus** | React | 跨平台(Web/桌面/移动) | ⭐⭐⭐ |
| **Sycamore** | Svelte | 无虚拟 DOM | ⭐⭐⭐ |

#### 示例: Yew 组件

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <h1>{ "Counter: " }{ *counter }</h1>
            <button onclick={increment}>{ "Increment" }</button>
        </div>
    }
}
```

### CLI 工具开发

#### 推荐 Crates

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }  # 命令行参数解析
tokio = { version = "1", features = ["full"] }     # 异步运行时
anyhow = "1.0"                                     # 错误处理
colored = "2.0"                                    # 彩色输出
indicatif = "0.17"                                 # 进度条
serde = { version = "1.0", features = ["derive"] } # 配置文件
```

#### 示例: 现代 CLI 工具

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A modern CLI tool", long_about = None)]
struct Cli {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("Processing: {}", cli.input);
}
```

---

## 🔧 开发工具和环境配置

### 安装 Rust

```bash
# 官方安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version

# 更新 Rust
rustup update
```

### 编辑器配置

#### VS Code (推荐)

必装插件:
1. **rust-analyzer** - 语言服务器 (比官方 rust 插件好)
2. **CodeLLDB** - 调试支持
3. **crates** - Cargo.toml 依赖管理
4. **Even Better TOML** - TOML 文件支持

配置 `settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

#### 其他编辑器

- **IntelliJ IDEA**: IntelliJ Rust 插件
- **Vim/Neovim**: rust.vim + coc-rust-analyzer
- **Sublime Text**: Rust Enhanced

### 常用 Cargo 命令

```bash
# 创建新项目
cargo new my-project      # 二进制项目
cargo new --lib my-lib    # 库项目

# 构建和运行
cargo build               # 调试构建
cargo build --release     # 发布构建 (优化)
cargo run                 # 构建并运行
cargo test                # 运行测试
cargo bench               # 性能基准测试

# 代码质量
cargo check               # 快速检查 (不生成可执行文件)
cargo clippy              # Lint 检查
cargo fmt                 # 代码格式化

# 依赖管理
cargo add <crate>         # 添加依赖 (需要 cargo-edit)
cargo update              # 更新依赖
cargo tree                # 查看依赖树

# 文档
cargo doc --open          # 生成并打开文档
```

### 实用工具

```bash
# 安装有用的 cargo 子命令
cargo install cargo-edit     # cargo add/rm/upgrade
cargo install cargo-watch    # 文件监听自动构建
cargo install cargo-expand   # 展开宏
cargo install cargo-outdated # 检查过时依赖
cargo install cargo-audit    # 安全审计
```

---

## 💡 学习技巧和常见陷阱

### 从 JavaScript 到 Rust 的思维转换

#### 1. 拥抱编译器错误

```javascript
// JavaScript: 运行时才发现错误
let user = null;
console.log(user.name); // Runtime error!
```

```rust
// Rust: 编译时就阻止你
let user: Option<User> = None;
// println!("{}", user.name); // 编译错误!

// 正确做法
match user {
    Some(u) => println!("{}", u.name),
    None => println!("No user"),
}
```

**建议**: 把编译器当作你最好的老师,仔细阅读错误信息。

#### 2. 理解所有权的"价值"

```javascript
// JavaScript: 到处复制引用,GC 处理
let a = [1, 2, 3];
let b = a;
let c = a; // 没问题
```

```rust
// Rust: 所有权转移
let a = vec![1, 2, 3];
let b = a;
// let c = a; // 错误! a 已被移动

// 解决方案1: 克隆
let a = vec![1, 2, 3];
let b = a.clone();
let c = a; // 现在可以

// 解决方案2: 借用
let a = vec![1, 2, 3];
let b = &a;
let c = &a; // 多个不可变借用 OK
```

#### 3. async/await 的不同

```javascript
// JavaScript: Promise-based
async function fetchData() {
  const response = await fetch(url);
  return response.json();
}
```

```rust
// Rust: 需要异步运行时 (Tokio)
async fn fetch_data() -> Result<Data, Error> {
    let response = reqwest::get(url).await?;
    response.json().await
}

// 需要在 #[tokio::main] 中运行
#[tokio::main]
async fn main() {
    let data = fetch_data().await.unwrap();
}
```

### 常见错误和解决方案

#### 错误 1: "cannot borrow as mutable"

```rust
// 错误代码
let x = vec![1, 2, 3];
x.push(4); // 错误: x 不可变

// 解决
let mut x = vec![1, 2, 3]; // 添加 mut
x.push(4);
```

#### 错误 2: "moved value"

```rust
// 错误代码
let s = String::from("hello");
let s2 = s;
println!("{}", s); // 错误: s 已被移动

// 解决方案
let s = String::from("hello");
let s2 = s.clone(); // 或使用 &s 借用
println!("{}", s);
```

#### 错误 3: 生命周期困惑

```rust
// 初学者常见错误
fn get_first(v: &Vec<String>) -> &String {
    &v[0] // 生命周期自动推导
}

// 当编译器无法推导时,需要显式标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 学习建议

1. **每天写代码** - Rust 需要肌肉记忆,每天 30 分钟比周末 4 小时效果好
2. **先理解所有权** - 这是最重要的概念,理解了就成功一半
3. **多读错误信息** - Rust 的错误信息是最好的教程
4. **加入社区** - Rust 社区非常友好,不要害怕提问
5. **对比学习** - 用 Rust 重写你熟悉的 Node.js 项目
6. **不要跳过 The Book** - 看起来长,但每一章都很重要

---

## 🌍 社区资源

### 官方论坛和聊天

- **Rust Users Forum**: https://users.rust-lang.org/
- **Rust Discord**: https://discord.gg/rust-lang
- **Reddit r/rust**: https://reddit.com/r/rust
- **Rust 中文社区**: https://rustcc.cn/

### 定期活动

- **Rust Weekly**: https://this-week-in-rust.org/ (每周新闻)
- **RustConf**: 年度官方会议
- **Rust Meetups**: 查找本地聚会

### 博客和新闻源

- **Rust Blog**: https://blog.rust-lang.org/
- **Read Rust**: https://readrust.net/ (精选文章)
- **Awesome Rust**: https://github.com/rust-unofficial/awesome-rust

### 练习平台

1. **Exercism** - https://exercism.org/tracks/rust
   - 163+ 练习 + 免费导师反馈

2. **LeetCode** - https://leetcode.com/
   - 支持 Rust 解题

3. **Advent of Code** - https://adventofcode.com/
   - 每年 12 月的编程挑战

---

## 🚀 30 天学习计划

### Week 1: Rust 基础

- **Day 1-2**: 安装环境 + The Book Ch1-3 (基础语法)
- **Day 3-4**: The Book Ch4-6 (所有权、结构体、枚举)
- **Day 5-6**: Rustlings 练习 (variables → functions)
- **Day 7**: 项目: 构建 CLI 计算器

### Week 2: 进阶特性

- **Day 8-9**: The Book Ch7-9 (模块、错误处理、泛型)
- **Day 10-11**: The Book Ch10-11 (trait、测试)
- **Day 12-13**: Rustlings 练习 (enums → traits)
- **Day 14**: 项目: JSON 解析器 CLI

### Week 3: Web 开发

- **Day 15-16**: Tokio 异步编程基础
- **Day 17-18**: Axum 框架入门
- **Day 19-20**: 构建 REST API
- **Day 21**: 项目: TODO API with SQLite

### Week 4: WebAssembly + 实战

- **Day 22-23**: wasm-pack + 浏览器集成
- **Day 24-25**: Yew 框架入门
- **Day 26-27**: 构建 WASM 应用
- **Day 28-30**: 最终项目: 全栈 Rust 应用

---

## 📊 Rust vs Node.js 性能对比

### 真实场景基准测试

| 场景 | Node.js | Rust | 性能提升 |
|------|---------|------|---------|
| JSON 解析 (大文件) | 1.2s | 0.08s | **15x** |
| HTTP 服务 (QPS) | 25k | 180k | **7x** |
| 图像处理 | 3.5s | 0.15s | **23x** |
| 正则匹配 (大文本) | 2.8s | 0.3s | **9x** |
| 文件搜索 (1GB) | 8s | 0.4s | **20x** |

**内存使用**: Rust 通常是 Node.js 的 1/5 - 1/10

### 何时选择 Rust?

✅ **Rust 更适合**:
- CPU 密集型任务 (编码、加密、压缩)
- 系统级工具 (CLI、守护进程)
- WebAssembly 应用
- 需要极致性能的微服务
- 嵌入式设备

✅ **Node.js 仍然更好**:
- 快速原型开发
- I/O 密集型应用
- 前端工具链 (Webpack、Babel)
- 需要快速迭代的项目
- 团队已有 JS 生态积累

**最佳实践**: 用 Node.js 快速验证想法,用 Rust 重写性能瓶颈部分。

---

## 🎓 认证和职业发展

### Rust 认证

虽然没有官方认证,但以下可以证明你的 Rust 能力:

1. **GitHub 开源贡献** - 为 Rust 项目提交 PR
2. **Crates.io 发布** - 发布自己的 crate
3. **技术博客** - 写 Rust 学习心得和实战经验
4. **Rustacean Station 播客嘉宾** - 分享你的项目

### 职业机会

**Rust 工程师需求增长领域**:

- 区块链开发 (Solana, Polkadot)
- 云原生基础设施 (AWS, Cloudflare)
- 游戏引擎 (Bevy)
- 浏览器引擎 (Firefox, Servo)
- 嵌入式系统
- 金融科技 (高频交易)

**薪资水平**: Rust 开发者平均薪资比 JavaScript 开发者高 20-40%

---

## 📝 总结和下一步

### 关键要点

1. **Rust 不是 JavaScript 的替代品** - 它们解决不同的问题
2. **投资学习曲线是值得的** - 一旦掌握,生产力极高
3. **社区是你最大的资产** - 不要独自学习
4. **从小项目开始** - CLI 工具是最好的起点
5. **拥抱编译器** - 它是你的老师,不是敌人

### 立即行动

1. **今天**: 安装 Rust + 完成 The Book 第 1-3 章
2. **本周**: 完成 Rustlings 前 20 个练习
3. **本月**: 构建一个简单的 CLI 工具
4. **三个月**: 构建一个 Web API 或 WASM 应用

### 保持联系

- 加入 Rust Discord 中文频道
- 关注 Rust 周报
- 参加本地 Rust Meetup

---

## 📌 快速参考卡片

### 常用类型对照

| JavaScript | Rust |
|-----------|------|
| `number` | `i32`, `f64` |
| `string` | `String`, `&str` |
| `boolean` | `bool` |
| `null/undefined` | `Option<T>` |
| `Array` | `Vec<T>` |
| `Object` | `HashMap<K,V>`, `struct` |
| `Promise` | `Future` |
| `async function` | `async fn` |

### 错误处理对照

```javascript
// JavaScript
try {
  const result = riskyOperation();
  return result;
} catch (error) {
  console.error(error);
  return null;
}
```

```rust
// Rust
match risky_operation() {
    Ok(result) => result,
    Err(e) => {
        eprintln!("Error: {}", e);
        return None;
    }
}

// 或使用 ? 操作符
let result = risky_operation()?;
```

### 循环对照

```javascript
// JavaScript
for (let item of items) {
  console.log(item);
}

items.forEach(item => console.log(item));
```

```rust
// Rust
for item in items {
    println!("{}", item);
}

items.iter().for_each(|item| println!("{}", item));
```

---

## 🔗 重要链接汇总

- 📘 **The Rust Book**: https://doc.rust-lang.org/book/
- 🎯 **Rustlings**: https://github.com/rust-lang/rustlings
- 💬 **Discord**: https://discord.gg/rust-lang
- 📦 **Crates.io**: https://crates.io/
- 📖 **Docs.rs**: https://docs.rs/
- 🎓 **Exercism**: https://exercism.org/tracks/rust
- 🌐 **Awesome Rust**: https://github.com/rust-unofficial/awesome-rust
- 📰 **This Week in Rust**: https://this-week-in-rust.org/

---

**祝你 Rust 学习之旅顺利! 🦀**

*最后更新: 2025-01*
*文档版本: 1.0*
*作者: Claude Code AI Assistant*
