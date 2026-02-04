# Cargo Test 输出多次测试日志的原因

## 问题

运行 `cargo test` 时，为什么会输出多次测试日志？

## 原因

这是 Rust/Cargo 的标准行为。`cargo test` 会运行**多个测试目标**，每个目标都有独立的输出。

## 测试目标说明

| 测试目标 | 说明 |
|---------|------|
| `Running unittests src/lib.rs` | lib.rs 中的单元测试（`#[test]`） |
| `Running unittests src/main.rs` | main.rs 中的单元测试 |
| `Running tests/health_check.rs` | `tests/` 目录下的集成测试 |
| `Doc-tests email` | 文档中的代码示例测试 |

## 示例输出

```
Running unittests src/lib.rs (target/debug/deps/email-xxx)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Running unittests src/main.rs (target/debug/deps/email-xxx)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Running tests/health_check.rs (target/debug/deps/health_check-xxx)
running 3 tests
test subscribe_returns_200_for_valid_form_data ... ok
test subscribe_returns_400_when_data_is_missing ... ok
test health_check_works ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests email
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 如何只运行特定测试

如果只想运行集成测试，可以使用以下命令：

```bash
# 只运行 health_check 集成测试
cargo test --test health_check

# 只运行库的单元测试
cargo test --lib

# 只运行文档测试
cargo test --doc

# 运行名称包含特定字符串的测试
cargo test health
```

## 运行特定模块的单元测试

### 单元测试 vs 集成测试的区别

| 类型 | 位置 | 运行方式 |
|------|------|----------|
| 单元测试 | `src/` 目录下，用 `#[cfg(test)]` 标记 | `cargo test --lib` 或按名称过滤 |
| 集成测试 | `tests/` 目录下，独立文件 | `cargo test --test <文件名>` |

### 项目结构示例

```
email/
├── src/
│   ├── lib.rs
│   └── domain/           ← 单元测试在这里 (#[cfg(test)])
│       ├── mod.rs
│       ├── subscriber_email.rs   ← 内含 #[cfg(test)] mod tests
│       └── subscriber_name.rs
└── tests/
    └── health_check.rs   ← 集成测试，可用 --test health_check
```

### 运行 `src/domain` 下的单元测试

```bash
# 方法1：按模块名过滤（推荐）
cargo test domain::

# 方法2：按具体子模块
cargo test domain::subscriber_email::
cargo test domain::subscriber_name::

# 方法3：按测试函数名
cargo test empty_string_is_rejected
cargo test valid_emails_are_parsed_successfully

# 方法4：只运行 lib 的单元测试（排除集成测试和文档测试）
cargo test --lib domain::

# 查看测试输出
cargo test domain:: -- --nocapture
```

### 常见误区

```bash
# 错误：--test 只能用于 tests/ 目录下的集成测试文件
cargo test --test domain   # ❌ 不会工作

# 正确：用名称过滤来运行 src/ 下的模块测试
cargo test domain::        # ✅ 运行 domain 模块的所有测试
```

---

## `cargo test --lib` 与 `cargo test` 的区别

### `cargo test email_client --lib`

只运行 **库代码 (src/)** 中匹配 `email_client` 的测试：

```
src/
├── lib.rs
├── email_client.rs  ← 只运行这里的 #[cfg(test)] 模块
└── ...
```

### `cargo test email_client`

运行**所有位置**中匹配 `email_client` 的测试：

```
src/                       ← 库测试
├── email_client.rs        ✅
tests/                     ← 集成测试
├── email_client.rs        ✅ (如果存在)
examples/                  ← 示例测试
benches/                   ← 基准测试
```

### 常用过滤选项

| 命令 | 作用 |
|------|------|
| `cargo test --lib` | 只运行 src/ 下的单元测试 |
| `cargo test --test health_check` | 只运行 tests/health_check.rs |
| `cargo test --bins` | 只运行 binary 中的测试 |
| `cargo test --doc` | 只运行文档测试 |

### TypeScript 类比

```bash
# Rust
cargo test --lib              # 类似只测 src/
cargo test --test integration # 类似只测 tests/

# Node.js (Jest)
jest src/                     # 只跑 src 目录
jest tests/                   # 只跑 tests 目录
```
