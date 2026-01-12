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
