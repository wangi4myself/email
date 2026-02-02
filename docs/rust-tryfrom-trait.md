# Rust TryFrom Trait 详解

## 示例代码

```rust
impl TryFrom<FormData> for NewSubscriber {
    type Error = String;  // 转换失败时的错误类型

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}
```

## 用 TypeScript 类比

```typescript
// Rust 的 TryFrom 类似于一个可能失败的构造函数/工厂方法

class NewSubscriber {
  email: SubscriberEmail;
  name: SubscriberName;

  // 类似 TryFrom 的静态工厂方法
  static tryFrom(formData: FormData): Result<NewSubscriber, string> {
    const nameResult = SubscriberName.parse(formData.name);
    if (nameResult.isErr()) {
      return Err(nameResult.error);
    }

    const emailResult = SubscriberEmail.parse(formData.email);
    if (emailResult.isErr()) {
      return Err(emailResult.error);
    }

    return Ok(new NewSubscriber(emailResult.value, nameResult.value));
  }
}

// 使用
const result = NewSubscriber.tryFrom(formData);
```

## 对照表

| Rust | TypeScript | 说明 |
|------|-----------|------|
| `TryFrom<T>` | 静态工厂方法 `tryFrom(t: T)` | 可能失败的类型转换 |
| `From<T>` | 构造函数 `new(t: T)` | 不会失败的类型转换 |
| `type Error = String` | 返回类型中的错误类型 | 定义失败时返回什么 |
| `value.try_into()` | `Class.tryFrom(value)` | 调用转换 |

## 使用方式

```rust
// 实现 TryFrom 后，自动获得 try_into() 方法
let new_subscriber: NewSubscriber = form.0.try_into()?;

// 等价于
let new_subscriber = NewSubscriber::try_from(form.0)?;
```

## From vs TryFrom

| Trait | 用途 | 示例 |
|-------|------|------|
| `From<T>` | 不会失败的转换 | `String::from("hello")` |
| `TryFrom<T>` | 可能失败的转换 | `i32::try_from(some_i64)?` |

## 为什么用 TryFrom？

| 方式 | 优点 |
|------|------|
| 独立函数 `parse_form(form)` | 简单直接 |
| `TryFrom` trait | 惯用法，可用 `try_into()`，与标准库一致 |

`TryFrom` 是 Rust 的惯用模式，表示"尝试从 A 类型转换到 B 类型，可能失败"。

## 自动实现 TryInto

当你实现 `TryFrom<A> for B` 时，Rust 自动为 `A` 实现 `TryInto<B>`：

```rust
// 你实现了这个
impl TryFrom<FormData> for NewSubscriber { ... }

// Rust 自动提供这个
impl TryInto<NewSubscriber> for FormData { ... }

// 所以你可以这样调用
let subscriber: NewSubscriber = form_data.try_into()?;
```

这类似于 TypeScript/JavaScript 中在原型链上添加方法，使得可以从实例上直接调用。
