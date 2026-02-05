# Rust 命令行开发指南

> 面向 Node.js 开发者的 Rust CLI 学习路径

## 官方推荐资源

### 1. Command Line Applications in Rust (官方书籍)

**地址**: https://rust-cli.github.io/book/

这是 Rust 官方推荐的 CLI 开发教程，完全免费。内容包括：

- CLI 参数解析
- 错误处理
- 输入输出
- 测试 CLI 应用
- 打包和分发

**类比 Node.js**: 类似于从零学习如何用 Node.js 写一个完整的 CLI 工具（不依赖 commander.js 等框架）。

---

## 核心库介绍

### clap - 命令行参数解析

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "一个示例 CLI 应用")]
struct Cli {
    /// 要处理的文件路径
    path: String,

    /// 启用详细模式
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    println!("处理文件: {}", args.path);
}
```

**Node.js 对比**:

| Rust          | Node.js              |
| ------------- | -------------------- |
| `clap`        | `commander` / `yargs` |
| `#[derive()]` | 装饰器模式            |
| 编译时验证    | 运行时验证            |

### 其他常用库

| 库名           | 用途                 | Node.js 等价物        |
| -------------- | -------------------- | --------------------- |
| `clap`         | 参数解析             | `commander`, `yargs`  |
| `indicatif`    | 进度条               | `ora`, `cli-progress` |
| `dialoguer`    | 交互式提示           | `inquirer`            |
| `console`      | 终端样式             | `chalk`               |
| `anyhow`       | 错误处理             | 自定义 Error 类       |
| `tokio`        | 异步运行时           | Node.js 内置事件循环  |

---

## 学习路径建议

### 阶段一：基础 (1-2 周)

1. **阅读官方 CLI Book 前 3 章**
   - 理解基本项目结构
   - 学习 clap 基础用法

2. **练习项目**: 实现一个简单的 `grep` 克隆
   ```bash
   cargo new minigrep
   ```

### 阶段二：进阶 (2-3 周)

1. **学习错误处理**
   - `anyhow` 用于应用程序
   - `thiserror` 用于库

2. **添加交互功能**
   - 使用 `dialoguer` 实现交互式菜单
   - 使用 `indicatif` 添加进度条

3. **练习项目**: 文件批量处理工具

### 阶段三：实战 (持续)

1. **学习异步 CLI**
   - 结合 `tokio` 处理网络请求
   - 并发任务处理

2. **分发与安装**
   - 使用 `cargo install` 分发
   - 通过 GitHub Releases 发布二进制

---

## 推荐项目参考

学习优秀开源项目的代码：

| 项目    | 功能         | 学习点                     |
| ------- | ------------ | -------------------------- |
| `ripgrep` | 快速搜索工具 | 性能优化、并行处理         |
| `bat`     | cat 替代品   | 语法高亮、终端输出         |
| `exa`     | ls 替代品    | 文件系统操作               |
| `fd`      | find 替代品  | 递归搜索、过滤             |
| `hyperfine` | 基准测试   | 统计分析、格式化输出       |

---

## 快速开始模板

```bash
# 创建新 CLI 项目
cargo new my-cli
cd my-cli

# 添加依赖
cargo add clap --features derive
cargo add anyhow
```

**Cargo.toml**:

```toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
```

---

## 与 Node.js CLI 开发对比

| 方面         | Node.js                     | Rust                        |
| ------------ | --------------------------- | --------------------------- |
| 启动速度     | 较慢 (需要 V8 启动)         | 极快 (原生二进制)           |
| 分发方式     | 需要 Node.js 环境           | 单一可执行文件              |
| 二进制大小   | N/A                         | 通常 1-10 MB                |
| 跨平台编译   | 天然跨平台                  | 需要交叉编译                |
| 开发速度     | 快速迭代                    | 编译等待                    |
| 类型安全     | TypeScript 可选             | 强制、编译时检查            |

---

## 总结

Rust 非常适合构建 CLI 工具，特别是需要高性能、零依赖分发的场景。对于 Node.js 开发者来说：

- **clap** 就像 TypeScript 版的 `commander`，但有更强的类型推导
- 错误处理模式（`Result<T, E>`）需要适应，但比 try-catch 更可靠
- 编译后的二进制可以直接分发，无需用户安装运行时
