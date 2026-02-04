# Rust 借用系统详解

## 引言

### 为什么需要借用？

在 TypeScript 中，对象默认通过引用传递：

```typescript
function modifyUser(user: User) {
  user.name = "Alice"; // 直接修改原对象
}

const user = { name: "Bob" };
modifyUser(user);
console.log(user.name); // "Alice" - 原对象被修改了
```

这种方式简单但存在隐患：
- 任何持有引用的代码都能修改数据
- 多线程环境下可能产生数据竞争
- 难以追踪数据何时何处被修改

Rust 的借用系统在编译时解决这些问题，让你既能高效地引用数据，又能保证内存安全。

### 所有权与借用的关系

| 概念 | TypeScript 类比 | Rust |
|------|----------------|------|
| 所有权 | 无直接对应（GC 管理） | 每个值有且只有一个所有者 |
| 移动 | 无（总是共享引用） | 赋值默认转移所有权 |
| 借用 | 对象引用 | `&T` 或 `&mut T` |

```rust
// 所有权转移
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权转移给 s2
// println!("{}", s1);  // 编译错误！s1 已无效

// 借用（不转移所有权）
let s1 = String::from("hello");
let s2 = &s1;  // 借用 s1
println!("{}", s1);  // OK，s1 仍然有效
println!("{}", s2);  // OK，通过借用访问
```

---

## 借用的基本概念

### 不可变借用 `&T`

不可变借用类似 TypeScript 的 `Readonly<T>`：只能读取，不能修改。

```rust
fn print_length(s: &String) {
    println!("长度: {}", s.len());
    // s.push_str("!"); // 编译错误！不能通过不可变借用修改
}

let message = String::from("Hello");
print_length(&message);  // 传递不可变借用
println!("{}", message); // message 仍然可用
```

**TypeScript 类比：**

```typescript
function printLength(s: Readonly<string[]>) {
  console.log(`长度: ${s.length}`);
  // s.push("!"); // TypeScript 错误！Readonly 不允许修改
}

const message = ["Hello"];
printLength(message);
console.log(message); // message 仍然可用
```

### 可变借用 `&mut T`

可变借用允许修改被借用的值，但有严格限制。

```rust
fn add_world(s: &mut String) {
    s.push_str(" World");
}

let mut message = String::from("Hello");
add_world(&mut message);
println!("{}", message); // "Hello World"
```

**TypeScript 类比：**

```typescript
function addWorld(s: { value: string }) {
  s.value += " World";
}

const message = { value: "Hello" };
addWorld(message);
console.log(message.value); // "Hello World"
```

### 借用 vs 所有权转移

| 操作 | 语法 | 效果 | TypeScript 类比 |
|------|------|------|----------------|
| 移动 | `let s2 = s1` | s1 失效 | 无（JS 不会使原变量失效） |
| 不可变借用 | `let s2 = &s1` | s1 仍有效，s2 只读 | `const ref: Readonly<T>` |
| 可变借用 | `let s2 = &mut s1` | s1 仍有效，s2 可写 | 普通对象引用 |
| 克隆 | `let s2 = s1.clone()` | 两者都有效 | `structuredClone(obj)` |

---

## 借用规则

### 规则一：互斥访问

> **同一时刻，要么只有一个可变引用，要么有任意数量的不可变引用，但不能同时存在。**

```rust
let mut s = String::from("hello");

// 情况 1：多个不可变借用 - OK
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2); // OK

// 情况 2：一个可变借用 - OK
let r3 = &mut s;
r3.push_str(" world"); // OK

// 情况 3：可变和不可变同时存在 - 编译错误！
let r1 = &s;
let r2 = &mut s;  // 错误！已存在不可变借用
println!("{}", r1);
```

**为什么需要这条规则？用 TypeScript 场景解释：**

```typescript
// 假设 TypeScript 有类似问题...
const users: string[] = ["Alice", "Bob"];

// 线程 1：遍历数组
for (const user of users) {
  console.log(user);
}

// 线程 2：同时修改数组
users.push("Charlie");
users.splice(0, 1);

// 可能导致：迭代器失效、读到脏数据、崩溃
```

Rust 在编译时阻止这种情况：

```rust
let mut users = vec!["Alice", "Bob"];

for user in &users {  // 不可变借用
    // users.push("Charlie"); // 编译错误！已被借用
    println!("{}", user);
}

users.push("Charlie"); // OK，循环结束后借用释放
```

### 规则二：引用必须始终有效

> **引用的生命周期不能超过被引用数据的生命周期。**

```rust
// 错误示例：悬垂引用
fn get_ref() -> &String {
    let s = String::from("hello");
    &s  // 编译错误！s 在函数结束时被销毁，返回的引用将无效
}

// 正确做法：返回所有权
fn get_string() -> String {
    let s = String::from("hello");
    s  // 转移所有权给调用者
}
```

**TypeScript 类比：**

TypeScript 因为有 GC，不会遇到这个问题：

```typescript
function getRef(): string {
  const s = "hello";
  return s; // OK，"hello" 不会被回收
}
```

但在使用闭包捕获变量时，概念相似：

```typescript
// 类似"悬垂引用"的问题
function createCallback() {
  let count = 0;
  return {
    increment: () => count++,
    getCount: () => count, // 闭包捕获了 count
  };
}
// count 因为被闭包引用，不会被回收
```

---

## 代码示例

### 基础借用示例

```rust
// 1. 函数参数借用
fn calculate_length(s: &String) -> usize {
    s.len()  // 借用 s，不获取所有权
}

let s = String::from("hello");
let len = calculate_length(&s);
println!("'{}' 的长度是 {}", s, len); // s 仍然可用

// 2. 方法中的 &self 和 &mut self
struct Counter {
    count: i32,
}

impl Counter {
    fn get(&self) -> i32 {        // 不可变借用 self
        self.count
    }

    fn increment(&mut self) {     // 可变借用 self
        self.count += 1;
    }
}
```

**TypeScript 类比：**

```typescript
// 类似的模式在 TypeScript 中
class Counter {
  private count: number = 0;

  get(): number {           // 类似 &self
    return this.count;
  }

  increment(): void {       // 类似 &mut self
    this.count += 1;
  }
}
```

### 借用检查器报错场景

**场景 1：同时存在可变和不可变借用**

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];     // 不可变借用
v.push(4);             // 错误！尝试可变借用
println!("{}", first); // first 仍在使用
```

编译器错误：
```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

**解决方案：**

```rust
let mut v = vec![1, 2, 3];
let first = v[0];      // 复制值（i32 实现了 Copy）
v.push(4);             // OK
println!("{}", first); // OK
```

**场景 2：在循环中修改集合**

```rust
let mut v = vec![1, 2, 3, 4, 5];
for x in &v {
    if *x > 2 {
        v.push(*x * 2);  // 错误！在遍历时修改
    }
}
```

**解决方案：**

```rust
let mut v = vec![1, 2, 3, 4, 5];
let additions: Vec<_> = v.iter().filter(|&&x| x > 2).map(|&x| x * 2).collect();
v.extend(additions);
```

### 实际项目中的借用模式

```rust
// 从配置中借用值
struct AppConfig {
    database_url: String,
    port: u16,
}

impl AppConfig {
    // 返回借用而非克隆，避免不必要的内存分配
    fn database_url(&self) -> &str {
        &self.database_url
    }
}

// 使用时
fn connect_database(config: &AppConfig) {
    let url = config.database_url();  // 借用，不复制
    // 使用 url 连接数据库...
}
```

---

## 与 TypeScript 的关键对比表

| 概念 | Rust | TypeScript | 说明 |
|------|------|-----------|------|
| 值传递 | 移动所有权 | 复制基本类型 | Rust 对象默认移动，TS 对象默认引用 |
| 引用传递 | `&T` 借用 | 对象引用 | Rust 需显式标记 |
| 可变性 | `&mut T` | 默认可变 | Rust 可变性需显式声明 |
| 只读引用 | `&T` | `Readonly<T>` | Rust 在编译时强制执行 |
| 生命周期 | 编译时检查 | GC 管理 | Rust 零运行时开销 |
| 并发安全 | 借用规则保证 | 无内置保证 | Rust 在编译时防止数据竞争 |
| 空指针 | `Option<&T>` | `T \| null` | Rust 必须显式处理 |

### 心智模型对照

| TypeScript 思维 | Rust 思维 |
|----------------|-----------|
| "这个对象会被 GC 回收" | "这个值的所有者负责释放它" |
| "任何地方都能修改这个对象" | "只有可变借用的持有者能修改" |
| "传对象进函数" | "是移动所有权还是借用？" |
| "返回对象的属性" | "返回借用还是克隆？" |

---

## 常见问题与解决方案

### 问题 1："cannot borrow as mutable"

```rust
let mut data = vec![1, 2, 3];
let first = &data[0];
data.push(4);  // 错误！
println!("{}", first);
```

**原因：** 同时存在不可变借用和可变操作。

**解决方案：**

```rust
// 方案 A：缩小借用范围
let mut data = vec![1, 2, 3];
{
    let first = &data[0];
    println!("{}", first);
}  // first 的借用在这里结束
data.push(4);  // OK

// 方案 B：使用索引而非引用
let mut data = vec![1, 2, 3];
let first_value = data[0];  // 复制值
data.push(4);
println!("{}", first_value);

// 方案 C：先完成所有读取
let mut data = vec![1, 2, 3];
let first = data[0];
let second = data[1];
data.push(4);
```

### 问题 2：悬垂引用

```rust
fn longest(x: &str, y: &str) -> &str {  // 错误！返回值生命周期不明确
    if x.len() > y.len() { x } else { y }
}
```

**解决方案：显式标注生命周期**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` 表示："返回值的生命周期与输入参数中较短的那个相同。"

### 问题 3：结构体中存储引用

```rust
// 错误：结构体持有引用但未标注生命周期
struct Excerpt {
    text: &str,
}

// 正确：显式标注生命周期
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(text: &'a str) -> Self {
        Excerpt { text }
    }
}
```

**TypeScript 类比：**

```typescript
// TypeScript 不需要考虑这个问题
interface Excerpt {
  text: string; // 引用不会失效，GC 会处理
}
```

---

## 总结

### Rust 借用的核心理念

1. **所有权明确**：每个值都有明确的所有者
2. **借用受控**：借用有明确的规则和范围
3. **编译时检查**：所有内存安全在编译时保证

### 从 TypeScript 到 Rust 的思维转变

| 习惯 | TypeScript | Rust |
|------|-----------|------|
| 传参 | 直接传对象 | 考虑：移动还是借用？ |
| 返回值 | 直接返回对象 | 考虑：返回所有权还是借用？ |
| 修改数据 | 随时修改 | 确保有可变借用且无其他借用 |
| 内存管理 | 交给 GC | 理解作用域和所有权 |

### 实用技巧

1. **优先使用借用**：避免不必要的克隆
2. **缩小借用范围**：借用用完就释放
3. **理解 Copy trait**：基本类型可自动复制
4. **善用 clone()**：不确定时先 clone，后续优化
5. **阅读编译器错误**：Rust 编译器提示非常有帮助
