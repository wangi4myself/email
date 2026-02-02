# Rust 模块导出模式详解

## 示例代码

```rust
// src/domain/mod.rs
mod new_subscriber;        // 声明子模块（引入 new_subscriber.rs）
mod subscriber_email;
mod subscriber_name;

pub use new_subscriber::NewSubscriber;      // 重新导出
pub use subscriber_email::SubscriberEmail;
pub use subscriber_name::SubscriberName;
```

## 用 TypeScript 类比

```typescript
// src/domain/index.ts
import { NewSubscriber } from './new_subscriber';      // 引入子模块
import { SubscriberEmail } from './subscriber_email';
import { SubscriberName } from './subscriber_name';

export { NewSubscriber };      // 重新导出
export { SubscriberEmail };
export { SubscriberName };

// 或者更简洁的写法：
export { NewSubscriber } from './new_subscriber';
export { SubscriberEmail } from './subscriber_email';
export { SubscriberName } from './subscriber_name';
```

## 对照表

| Rust | TypeScript | 作用 |
|------|-----------|------|
| `mod foo;` | `import ... from './foo'` | 引入子模块 |
| `pub use foo::Bar;` | `export { Bar } from './foo'` | 重新导出 |
| `mod.rs` | `index.ts` | 目录入口文件 |

## 使用效果

### Rust

```rust
// 可以从 domain 直接导入
use crate::domain::SubscriberName;

// 而不需要写完整路径
use crate::domain::subscriber_name::SubscriberName;
```

### TypeScript

```typescript
// 可以从 domain 直接导入
import { SubscriberName } from './domain';

// 而不需要写完整路径
import { SubscriberName } from './domain/subscriber_name';
```

## 这种模式的好处

1. **隐藏内部结构** - 外部使用者只需要知道 `domain` 模块，不需要关心里面有哪些文件
2. **重构友好** - 可以自由调整内部文件结构，只要保持导出不变
3. **API 清晰** - 明确控制哪些类型对外暴露

## Rust 模块系统要点

- `mod foo;` 声明模块，Rust 会查找 `foo.rs` 或 `foo/mod.rs`
- 默认私有，需要 `pub` 才能对外可见
- `pub use` 是重新导出（re-export），类似 TS 的 `export { } from`
