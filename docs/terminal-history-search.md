# 终端历史命令查找指南

在终端中查找历史命令有多种方法，以下是常用的几种方式。

## 1. 使用 `history` 命令

```bash
# 查看所有历史命令
history

# 查看最近 20 条命令
history 20

# 配合 grep 搜索特定命令
history | grep docker
```

## 2. 使用快捷键 `Ctrl + R`

按下 `Ctrl + R` 进入反向搜索模式，输入关键字即可搜索：

- 继续按 `Ctrl + R` 查找上一个匹配项
- 按 `Enter` 执行找到的命令
- 按 `Ctrl + G` 或 `Esc` 退出搜索

这是最常用且高效的方式。

## 3. 使用方向键

- `↑` 上一条命令
- `↓` 下一条命令

适合快速查找最近执行过的命令。

## 4. 使用 `!` 快捷方式

```bash
# 执行上一条命令
!!

# 执行历史中第 100 条命令
!100

# 执行最近以 docker 开头的命令
!docker

# 查看最近以 git 开头的命令（不执行）
!git:p
```

## 5. 直接搜索历史文件

```bash
# Bash 用户
grep "关键字" ~/.bash_history

# Zsh 用户
grep "关键字" ~/.zsh_history
```

## 注意事项

- macOS 默认使用 **zsh**，历史文件位于 `~/.zsh_history`
- Linux 大多使用 **bash**，历史文件位于 `~/.bash_history`
- `Ctrl + R` 是最推荐的搜索方式，快速且高效
