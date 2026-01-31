# Rust 嵌入式开发学习路径

> 从 Rust 初学者到嵌入式开发的完整学习指南

## 学习路线图

```
阶段1: Rust基础        阶段2: 嵌入式基础       阶段3: 实战项目
     │                      │                      │
     ▼                      ▼                      ▼
┌─────────────┐      ┌─────────────┐       ┌─────────────┐
│ 所有权/借用  │      │  no_std 环境 │       │  LED/按键   │
│ 生命周期    │ ───► │  寄存器操作  │ ────► │  传感器读取  │
│ 错误处理    │      │  中断/定时器 │       │  BLE 通信   │
│ 泛型/Trait  │      │  embedded-hal│       │  完整项目   │
└─────────────┘      └─────────────┘       └─────────────┘
    2-3个月              1-2个月               持续实践
```

---

## 阶段一：Rust 语言基础（必修）

### 1.1 核心概念（优先级最高）

| 概念 | 重要性 | 说明 |
|------|--------|------|
| 所有权系统 | ⭐⭐⭐⭐⭐ | Rust的核心，嵌入式中尤为重要 |
| 借用与生命周期 | ⭐⭐⭐⭐⭐ | 理解引用如何工作 |
| Option/Result | ⭐⭐⭐⭐⭐ | 嵌入式错误处理的基础 |
| Trait | ⭐⭐⭐⭐ | embedded-hal 全靠它 |
| 泛型 | ⭐⭐⭐⭐ | 编写可复用的硬件抽象 |
| 模式匹配 | ⭐⭐⭐ | match 表达式 |
| 闭包 | ⭐⭐⭐ | 中断回调常用 |

### 1.2 推荐学习资源

**官方资源（免费）**
- [The Rust Book](https://doc.rust-lang.org/book/) - 官方教程，必读
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例驱动学习
- [Rustlings](https://github.com/rust-lang/rustlings) - 交互式练习题

**视频教程**
- [Rust 程序设计语言 - B站](https://www.bilibili.com/video/BV1hp4y1k7SV) - 中文讲解
- [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty) - 英文YouTube频道

### 1.3 练习项目建议

```rust
// 在进入嵌入式之前，确保能独立完成这些：
// 1. 实现一个简单的命令行工具
// 2. 用 Option/Result 处理文件读取错误
// 3. 定义 Trait 并为多个类型实现
// 4. 理解这段代码为什么编译失败：
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1); // 为什么报错？
}
```

---

## 阶段二：嵌入式 Rust 基础

### 2.1 嵌入式的特殊之处

```rust
// 普通 Rust 程序
fn main() {
    println!("Hello, world!"); // 依赖操作系统
}

// 嵌入式 Rust (no_std)
#![no_std]  // 不使用标准库
#![no_main] // 不使用标准入口点

#[entry]
fn main() -> ! {
    loop {
        // 直接操作硬件
    }
}
```

### 2.2 必学概念

| 概念 | 说明 |
|------|------|
| `#![no_std]` | 不使用标准库，只用 `core` 库 |
| `#![no_main]` | 自定义程序入口 |
| 寄存器操作 | 通过内存地址控制硬件 |
| 中断处理 | 响应硬件事件 |
| 外设抽象 | GPIO、UART、SPI、I2C 等 |

### 2.3 核心文档（必读）

1. **[The Embedded Rust Book](https://docs.rust-embedded.org/book/)**
   - 官方嵌入式入门书，从零开始讲解
   - 重点章节：Hardware、Portability、Concurrency

2. **[Discovery Book](https://docs.rust-embedded.org/discovery/)**
   - 基于 micro:bit 或 STM32F3 的实践教程
   - 手把手带你点亮第一个 LED

3. **[Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)**
   - 深入理解嵌入式 Rust 底层原理
   - 进阶阅读

### 2.4 关键 Crate 生态

```toml
# Cargo.toml 中常见的嵌入式依赖

[dependencies]
# 核心抽象层
embedded-hal = "1.0"        # 硬件抽象层 Trait
cortex-m = "0.7"            # ARM Cortex-M 支持
cortex-m-rt = "0.7"         # 运行时（启动代码）

# 具体芯片支持（选一个）
stm32f4xx-hal = "0.21"      # STM32F4 系列
nrf52840-hal = "0.18"       # Nordic nRF52840
esp32-hal = "0.20"          # ESP32 系列

# 常用工具
panic-halt = "0.2"          # panic 处理
defmt = "0.3"               # 高效调试打印
```

---

## 阶段三：硬件平台选择

### 3.1 推荐开发板对比

| 开发板 | 价格 | 优势 | 适合场景 |
|--------|------|------|----------|
| **STM32F4-Discovery** | ¥150 | 资料丰富，社区活跃 | 入门首选 |
| **nRF52840-DK** | ¥300 | 蓝牙BLE内置 | 无线通信项目 |
| **ESP32-C3** | ¥30 | 便宜，WiFi+BLE | IoT项目 |
| **Raspberry Pi Pico** | ¥25 | RP2040双核 | 性价比高 |
| **micro:bit v2** | ¥100 | 传感器丰富 | 教学入门 |

### 3.2 我的建议

**初学者首选：ESP32-C3**
- 价格低，试错成本小
- Rust 支持完善（esp-rs 项目）
- 自带 WiFi 和 BLE
- 可以后续做物联网项目

```bash
# 安装 ESP32 Rust 工具链
cargo install espup
espup install

# 创建新项目
cargo generate esp-rs/esp-template
```

**进阶推荐：STM32F4 或 nRF52840**
- 更标准的 ARM Cortex-M 架构
- 学习价值更高

---

## 阶段四：实战项目路线

### 4.1 循序渐进的项目列表

```
Level 1: 基础 IO
├── LED 闪烁 (GPIO 输出)
├── 按键检测 (GPIO 输入)
└── PWM 控制 LED 亮度

Level 2: 通信协议
├── UART 串口通信
├── I2C 读取温度传感器
└── SPI 驱动 OLED 屏幕

Level 3: 中断与定时
├── 定时器中断精确计时
├── 外部中断响应按键
└── 实现软件去抖动

Level 4: 无线通信
├── BLE 广播数据
├── BLE 连接手机 APP
└── WiFi HTTP 请求

Level 5: 综合项目
├── 智能温湿度监控器
├── BLE 遥控小车
└── 手机解锁模拟系统 ← 你的目标
```

### 4.2 第一个项目：LED 闪烁

```rust
// ESP32-C3 示例
#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay,
};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio8.into_push_pull_output();

    let mut delay = Delay::new(&clocks);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}
```

---

## 阶段五：调试与工具链

### 5.1 必备工具

```bash
# 基础工具
rustup target add thumbv7em-none-eabihf  # ARM 编译目标
cargo install probe-rs                    # 调试/烧录工具
cargo install cargo-embed                 # 嵌入式cargo扩展

# ESP32 专用
cargo install espflash                    # ESP 烧录工具
cargo install espmonitor                  # 串口监视器
```

### 5.2 调试技巧

```rust
// 使用 defmt 进行高效日志输出
use defmt::info;

#[entry]
fn main() -> ! {
    info!("程序启动");

    loop {
        info!("当前温度: {} °C", read_temperature());
    }
}
```

---

## 学习资源汇总

### 在线社区

| 资源 | 链接 | 说明 |
|------|------|------|
| Rust 嵌入式工作组 | https://github.com/rust-embedded | 官方组织 |
| Awesome Embedded Rust | https://github.com/rust-embedded/awesome-embedded-rust | 资源合集 |
| ESP-RS | https://github.com/esp-rs | ESP32 Rust 支持 |
| Embassy | https://embassy.dev | 异步嵌入式框架 |

### 书籍推荐

1. 《Rust 程序设计语言》 - 先打好 Rust 基础
2. 《The Embedded Rust Book》 - 在线免费
3. 《Programming Rust, 2nd Edition》 - 深入理解

### 视频/博客

- [Ferrous Systems 嵌入式培训](https://github.com/ferrous-systems/embedded-trainings-2020)
- [James Munns 的博客](https://jamesmunns.com/) - 嵌入式 Rust 专家
- [The Rusty Bits - YouTube](https://www.youtube.com/c/TheRustyBits)

---

## 学习时间线建议

| 阶段 | 时间 | 目标 |
|------|------|------|
| Rust 基础 | 2-3 个月 | 完成 Rust Book，能写 CLI 工具 |
| 嵌入式入门 | 1 个月 | 完成 Discovery Book，点亮 LED |
| 外设驱动 | 1-2 个月 | 掌握 UART/I2C/SPI |
| 无线通信 | 1-2 个月 | 实现 BLE 或 WiFi 通信 |
| 综合项目 | 持续 | 构建完整应用 |

---

## 下一步行动

1. **今天**：安装 Rust，完成 Rustlings 前 10 题
2. **本周**：读完 Rust Book 第 1-6 章
3. **本月**：购买一块 ESP32-C3 开发板
4. **下月**：完成第一个 LED 闪烁程序

祝你学习顺利！嵌入式 + Rust 是一个非常有前景的方向。
