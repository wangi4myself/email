# Rust String 与 &str 互相转换

## 背景：为什么用借用引用 `&str`

在 `SendEmailRequest` 结构体中使用 `&'a str` 而非 `String`：

```rust
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}
```

### 好处

#### 1. 零拷贝 (Zero-Copy)

```rust
// 当前写法：只借用，不复制
struct SendEmailRequest<'a> {
    from: &'a str,      // 借用外部字符串
}

// 对比：拥有数据，需要复制
struct SendEmailRequestOwned {
    from: String,       // 需要 .to_string() 或 .clone()
}
```

**TypeScript 类比**：想象一下如果 JS 每次传递字符串都要深拷贝，而不是传引用——性能会很差。Rust 的 `&str` 就像 TS 中直接传递字符串引用。

#### 2. 性能优势

这个结构体用于构建 HTTP 请求体，是**短生命周期**的临时对象：

```rust
let request_body = SendEmailRequest {
    from: self.sender.as_ref(),  // 不分配新内存
    to: recipient.as_ref(),       // 只是指向已有数据
    // ...
};
// 序列化后 request_body 就丢弃了
```

#### 3. `'a` 的含义

`'a` 是**生命周期参数**，告诉编译器：所有借用的数据必须活得比 `SendEmailRequest` 更久。

```rust
// 编译器保证：sender 和 recipient 的生命周期 >= request_body
let request_body = SendEmailRequest {
    from: self.sender.as_ref(),  // sender 必须还活着
    to: recipient.as_ref(),       // recipient 必须还活着
    // ...
};
```

### 何时用 `&str` vs `String`

| 场景 | 选择 | 原因 |
|------|------|------|
| 临时结构体（如 HTTP 请求） | `&str` | 数据来自外部，用完即弃 |
| 需要长期存储 | `String` | 需要拥有数据所有权 |
| API 返回值 | `String` | 不能返回局部变量的引用 |

---

## String → &str

```rust
let s: String = String::from("hello");

// 方法 1：自动解引用 (最常用)
let slice: &str = &s;

// 方法 2：显式调用
let slice: &str = s.as_str();

// 方法 3：切片语法
let slice: &str = &s[..];
```

## &str → String

```rust
let slice: &str = "hello";

// 方法 1：.to_string()
let s: String = slice.to_string();

// 方法 2：.to_owned()
let s: String = slice.to_owned();

// 方法 3：String::from()
let s: String = String::from(slice);

// 方法 4：.into()
let s: String = slice.into();
```

## 速查表

| 转换方向 | 代码 | 是否分配内存 |
|---------|------|-------------|
| `String` → `&str` | `&s` 或 `s.as_str()` | 否 (零成本) |
| `&str` → `String` | `s.to_string()` | 是 (堆分配) |

## TypeScript 类比

```typescript
// TS 中没有这个区别，但可以这样理解：
const owned = "hello"           // String - 拥有数据
const borrowed = owned          // &str - 只是引用，TS 自动处理

// Rust 需要显式：
let owned: String = String::from("hello");  // 堆上分配
let borrowed: &str = &owned;                 // 栈上指针，指向 owned 的数据
```

## 实际场景

```rust
fn send_email(to: &str) {       // 接收借用，更灵活
    // ...
}

let email_owned: String = get_email_from_db();
let email_literal: &str = "test@example.com";

// 两种类型都能传入
send_email(&email_owned);       // String → &str
send_email(email_literal);      // &str 直接传
```

## 经验法则

- **函数参数**用 `&str`（接收方灵活，调用者可传 String 或 &str）
- **结构体存储**用 `String`（需要拥有数据所有权）
- **临时使用**用 `&str`（避免不必要的堆分配）
