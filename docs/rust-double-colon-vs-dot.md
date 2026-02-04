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

**TypeScript 类比：**
```typescript
// TS 使用 import 语句，用 / 或相对路径
import { HashMap } from 'std/collections';
import { subscriptions } from './routes';

// 或者命名空间方式（用 . 而非 ::）
namespace std {
  export namespace collections {
    export class HashMap<K, V> { }
  }
}
const map = new std.collections.HashMap();  // TS 用 .
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

**TypeScript 类比：**
```typescript
// TS 静态方法用 static 关键字，但调用时用 .
const arr = Array.from([1, 2, 3]);
const obj = Object.create(null);

class User {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
    // 静态方法（对应 Rust 关联函数）
    static new(name: string): User {
        return new User(name);
    }
}

const user = User.new("Alice");  // TS 用 . 而 Rust 用 ::
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

**TypeScript 类比：**
```typescript
enum Color {
    Red,
    Green,
    Blue,
}

const c = Color.Red;  // TS 用 . 而 Rust 用 ::

// TS 没有内置 Option，通常用联合类型或命名空间模拟
type Option<T> = { kind: 'Some'; value: T } | { kind: 'None' };

namespace Option {
    export function Some<T>(value: T) { return { kind: 'Some' as const, value }; }
    export function None<T>() { return { kind: 'None' as const }; }
}

const someValue = Option.Some(5);  // TS 用 .
```

### 4. 访问关联常量

```rust
struct Circle;

impl Circle {
    const PI: f64 = 3.14159;
}

println!("{}", Circle::PI);  // 使用 ::
```

**TypeScript 类比：**
```typescript
class Circle {
    static readonly PI: number = 3.14159;
}

console.log(Circle.PI);  // TS 用 . 而 Rust 用 ::
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

**TypeScript 类比：**
```typescript
// TS 泛型参数直接用 <> 跟在函数名后，不需要 ::
function parse<T>(s: string): T { /* ... */ }
const num = parse<number>("42");  // 用 <> 而非 ::<>

const numbers: Array<number> = [1, 2, 3];
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

**TypeScript 类比：**
```typescript
const s = "hello";

// TS 实例方法同样用 .
const len = s.length;              // 实例属性
const upper = s.toUpperCase();     // 实例方法
// ✅ 两者语法相同
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

**TypeScript 类比：**
```typescript
interface Point {
    x: number;
    y: number;
}

const p: Point = { x: 10, y: 20 };
console.log("x =", p.x);  // ✅ 相同，都用 .
console.log("y =", p.y);
```

### 3. 链式调用

```rust
let result = "  hello world  "
    .trim()           // 实例方法
    .to_uppercase()   // 实例方法
    .replace(" ", "_"); // 实例方法
```

**TypeScript 类比：**
```typescript
const result = "  hello world  "
    .trim()
    .toUpperCase()
    .replace(" ", "_");
// ✅ 完全相同的语法
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

**TypeScript 类比：**
```typescript
class MyStruct {
    // static → 静态方法 → 通过类名.调用
    static new(): MyStruct { return new MyStruct(); }

    // 非 static → 实例方法 → 通过实例.调用
    doSomething(): void { }
}

const obj = MyStruct.new();  // . (通过类名调用)
obj.doSomething();           // . (通过实例调用)
// TS 都用 .，但 Rust 用 :: 和 . 区分
```

### 快速记忆法

| 场景 | Rust | TypeScript |
|------|------|------------|
| 需要实例才能调用 | `.` | `.` |
| 不需要实例，直接通过类型调用 | `::` | `.` (static) |
| 访问模块/命名空间 | `::` | `.` 或 import |
| 访问字段/属性 | `.` | `.` |

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

**TypeScript 类比：**
```typescript
import { HashMap } from 'collections';  // import 访问模块

function main() {
    // 静态方法创建实例
    const map = new Map<string, string>();

    // . 调用实例方法
    map.set("key", "value");

    // . 访问"枚举"
    const option = Option.Some(42);

    // . 调用实例方法
    const value = option.value;

    // 泛型参数用 <>
    const numbers: Array<number> = [1, 2, 3];
    const first = numbers[0];

    // 都用 .
    const s = String.fromCharCode(104, 101, 108, 108, 111);
    const len = s.length;
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

**TypeScript 类比：**
```typescript
// TS 没有显式引用类型，对象本身就是引用
const s = "hello";
const r = s;  // 只是另一个变量指向同一个值

s.length;  // 直接访问
r.length;  // 相同

// TS 没有解引用概念，因为没有 & 引用类型
```

---

## 总结对比

| 概念 | Rust | TypeScript |
|------|------|------------|
| 模块访问 | `::` | `import` / `.` |
| 静态方法 | `Type::method()` | `Type.method()` |
| 枚举变体 | `Enum::Variant` | `Enum.Variant` |
| 静态常量 | `Type::CONST` | `Type.CONST` |
| 泛型参数 | `::<T>` | `<T>` |
| 实例方法 | `.method()` | `.method()` |
| 字段访问 | `.field` | `.field` |

**核心差异**：TypeScript 统一使用 `.`，而 Rust 通过 `::` 和 `.` 的区分让代码更显式地表达"这是类型级操作还是实例级操作"。
