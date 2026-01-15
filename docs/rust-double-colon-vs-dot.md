# Rust 中 `::` 与 `.` 的使用区别

## 概述

在 Rust 中，`::` 和 `.` 是两个不同的操作符，用于不同的访问场景：

| 操作符 | 名称 | 用途 |
|--------|------|------|
| `::` | 路径操作符 | 访问模块、类型、关联函数/常量 |
| `.` | 点操作符 | 访问实例的方法和字段 |

---

## `::` 双冒号 (Path Operator)

### 1. 访问模块和子模块

```rust
use std::collections::HashMap;
use crate::routes::subscriptions;
```

### 2. 调用关联函数（静态方法）

关联函数不需要实例，直接通过类型调用：

```rust
// String 的关联函数
let s = String::new();
let s = String::from("hello");

// Vec 的关联函数
let v: Vec<i32> = Vec::new();
let v = Vec::with_capacity(10);

// 自定义结构体
struct User {
    name: String,
}

impl User {
    // 关联函数（没有 self 参数）
    fn new(name: &str) -> Self {
        User { name: name.to_string() }
    }
}

let user = User::new("Alice");  // 使用 ::
```

### 3. 访问枚举变体

```rust
enum Color {
    Red,
    Green,
    Blue,
}

let c = Color::Red;  // 使用 ::

// Option 和 Result 也是枚举
let some_value = Option::Some(5);
let ok_value = Result::Ok::<i32, String>(42);
```

### 4. 访问关联常量

```rust
struct Circle;

impl Circle {
    const PI: f64 = 3.14159;
}

println!("{}", Circle::PI);  // 使用 ::
```

### 5. 指定泛型类型（turbofish 语法）

```rust
let numbers: Vec<i32> = "1,2,3"
    .split(',')
    .map(|s| s.parse::<i32>().unwrap())  // ::<i32> 指定类型
    .collect();

// 或者
let parsed = "42".parse::<i32>().unwrap();
```

---

## `.` 点操作符 (Dot Operator)

### 1. 调用实例方法

实例方法的第一个参数是 `self`、`&self` 或 `&mut self`：

```rust
let s = String::from("hello");

// 这些都是实例方法，需要通过实例调用
let len = s.len();           // &self
let upper = s.to_uppercase(); // &self
s.push_str(" world");        // &mut self（需要 s 是 mut）
```

### 2. 访问结构体字段

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 10, y: 20 };
println!("x = {}", p.x);  // 使用 .
println!("y = {}", p.y);
```

### 3. 链式调用

```rust
let result = "  hello world  "
    .trim()           // 实例方法
    .to_uppercase()   // 实例方法
    .replace(" ", "_"); // 实例方法
```

---

## 关键区别总结

### 看函数签名判断

```rust
impl MyStruct {
    // 关联函数：没有 self → 用 ::
    fn new() -> Self { ... }

    // 实例方法：有 self → 用 .
    fn do_something(&self) { ... }
}

// 调用方式
let obj = MyStruct::new();      // :: 因为 new() 没有 self
obj.do_something();             // .  因为 do_something 有 &self
```

### 快速记忆法

| 场景 | 使用 | 示例 |
|------|------|------|
| 需要实例才能调用 | `.` | `instance.method()` |
| 不需要实例，直接通过类型调用 | `::` | `Type::function()` |
| 访问模块/命名空间 | `::` | `std::io::Result` |
| 访问字段 | `.` | `struct.field` |

---

## 实际代码示例

```rust
use std::collections::HashMap;  // :: 访问模块

fn main() {
    // :: 调用关联函数创建实例
    let mut map = HashMap::new();

    // . 调用实例方法
    map.insert("key", "value");

    // :: 访问枚举变体
    let option = Option::Some(42);

    // . 调用实例方法
    let value = option.unwrap();

    // 泛型方法中的 turbofish
    let numbers: Vec<i32> = vec![1, 2, 3];
    let first = numbers.get(0);  // . 实例方法

    // :: 用于类型路径
    let s = String::from("hello");  // :: 关联函数
    let len = s.len();              // .  实例方法
}
```

---

## 特殊情况：自动解引用

Rust 的 `.` 操作符会自动进行解引用（deref coercion），这使得代码更简洁：

```rust
let s = String::from("hello");
let r = &s;
let rr = &&s;

// 以下三种调用都有效，Rust 自动解引用
println!("{}", s.len());   // 直接调用
println!("{}", r.len());   // 自动解引用 &String
println!("{}", rr.len());  // 自动多次解引用 &&String
```

而 `::` 不会自动解引用，它是静态路径解析。
