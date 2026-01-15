# Rust impl Trait 返回类型详解

## 背景代码

```rust
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync
```

## 拆解理解

### 1. `impl Trait` - 不透明返回类型

意思是：**"返回某个实现了这些 trait 的类型，但不告诉你具体是什么类型"**

```rust
// 实际返回的是这个复杂类型：
Registry<Layered<BunyanFormattingLayer<...>, Layered<JsonStorageLayer, ...>>>

// 但写出来太长了，所以用 impl Trait 隐藏具体类型
fn get_subscriber() -> impl Subscriber + Send + Sync
```

### 2. `+` - 多个 trait 约束

表示返回的类型必须**同时实现**所有这些 trait：

| Trait        | 含义                             |
| ------------ | -------------------------------- |
| `Subscriber` | 日志订阅者，能接收 tracing 事件  |
| `Send`       | 可以安全地跨线程**传递**         |
| `Sync`       | 可以安全地跨线程**共享引用**     |

### 3. 为什么需要 `Send + Sync`？

```rust
// init_subscriber 的签名
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync)
```

`set_global_default` 会将 subscriber 设为全局的，多线程都会访问它，所以必须是线程安全的。

## 四种返回类型方式对比

```rust
// 方式 1: impl Trait（推荐）
// 优点：简洁，零开销，类型由函数实现决定
fn get_subscriber() -> impl Subscriber + Send + Sync { ... }

// 方式 2: 泛型（调用者决定类型）
// 不适用于此场景，因为类型由函数内部决定
fn get_subscriber<S: Subscriber + Send + Sync>() -> S { ... }  // ❌

// 方式 3: trait object（动态分发）
// 缺点：有运行时开销（vtable 查找）
fn get_subscriber() -> Box<dyn Subscriber + Send + Sync> { ... }

// 方式 4: 写出具体类型
// 缺点：太长，且内部实现改变时签名也要改
fn get_subscriber() -> Layered<BunyanFormattingLayer<Stdout>,
                              Layered<JsonStorageLayer,
                                     Filtered<Registry, EnvFilter>>> { ... }
```

## 图解总结

```
impl Subscriber + Send + Sync
│    │            │      │
│    │            │      └── 可跨线程共享引用
│    │            └── 可跨线程传递所有权
│    └── 实现了 Subscriber trait
└── "返回一个实现了...的类型"（具体类型由编译器推断，对调用者隐藏）
```

## impl Trait 的特点

| 特点           | 说明                                         |
| -------------- | -------------------------------------------- |
| 零成本抽象     | 编译时确定具体类型，无运行时开销             |
| 类型隐藏       | 调用者不需要知道具体类型                     |
| 简化签名       | 避免写出冗长的泛型类型                       |
| 编译器推断     | 具体类型由编译器根据函数体推断               |
| 单一具体类型   | 每次调用返回相同的具体类型（不同于 dyn Trait）|

## 使用场景

1. **返回闭包**
   ```rust
   fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
       move |y| x + y
   }
   ```

2. **返回迭代器**
   ```rust
   fn double_values(v: Vec<i32>) -> impl Iterator<Item = i32> {
       v.into_iter().map(|x| x * 2)
   }
   ```

3. **返回复杂组合类型**（如本例的 tracing subscriber）
   ```rust
   fn get_subscriber() -> impl Subscriber + Send + Sync {
       Registry::default()
           .with(env_filter)
           .with(JsonStorageLayer)
           .with(formatting_layer)
   }
   ```
