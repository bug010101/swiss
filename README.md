<div align="center">

# swiss

**瑞士军刀式开发小工具箱**

时间戳 / Base64 / URL / JSON / 命名转换 / 哈希 / UUID，一条命令搞定。

</div>

## 安装

### 从 GitHub Releases 下载（推荐）

到 [Releases 页面](../../releases) 下载对应平台的包：

| 平台 | 文件 |
|---|---|
| macOS（Apple Silicon，M1/M2/M3/M4） | `swiss-aarch64-apple-darwin.tar.gz` |
| Windows x64 | `swiss-x86_64-pc-windows-msvc.zip` |
| Linux x64（Ubuntu / Debian / Fedora 等主流发行版） | `swiss-x86_64-unknown-linux-gnu.tar.gz` |

每个 Release 附带 `checksums.txt`（SHA-256 校验和），下载后建议验证：

```sh
# macOS / Linux
shasum -a 256 -c --ignore-missing checksums.txt

# Windows (PowerShell)
Get-FileHash swiss-*.zip -Algorithm SHA256
```

**macOS 解压安装**：

```sh
tar xzf swiss-aarch64-apple-darwin.tar.gz
sudo mkdir -p /usr/local/bin
sudo mv swiss /usr/local/bin/
swiss --help
```

> 也可以放到 `~/.local/bin` 或 `/opt/homebrew/bin` 等已在 PATH 的目录。

**Windows 解压安装**：解压 `zip` 得到 `swiss.exe`，建议放到专用目录（如 `C:\Tools\swiss\`），再把该目录加入 PATH（设置 → 系统 → 高级系统设置 → 环境变量 → Path → 编辑 → 新建），然后新开终端运行 `swiss --help`。

**Linux 解压安装**：

```sh
tar xzf swiss-x86_64-unknown-linux-gnu.tar.gz
sudo mv swiss /usr/local/bin/
swiss --help
```

### 其他 Linux 架构 / 发行版（源码构建）

官方 Release 只提供 x64 (glibc) 版本。其他环境的用户请从源码构建：

```sh
# 需要 Rust 工具链：https://rustup.rs
cargo install --git https://github.com/<你的用户名>/swiss.git
```

或克隆后本地安装：

```sh
git clone https://github.com/<你的用户名>/swiss.git
cd swiss
cargo install --path .
```

常见环境对应关系：

| 环境 | 构建方式 |
|---|---|
| x64 + glibc（绝大多数发行版） | 直接下载 Release 包 |
| x64 + musl（Alpine 等） | `cargo install --git ...`（rustup 默认 target 即可） |
| ARM64（树莓派、ARM 服务器、国产 ARM 盒子） | `cargo install --git ...` |
| 其他架构 | `cargo install --git ...` |

> 从源码构建需要有对应平台的 Rust 工具链，参见 [rustup.rs](https://rustup.rs)。

## 使用

### 🕐 时间戳互转

```sh
swiss ts                      # 当前时间（本地 / UTC / 秒 / 毫秒）
swiss ts 1762671000           # 秒级时间戳 -> 日期时间
swiss ts 1762671000000        # 毫秒级时间戳 -> 日期时间
swiss ts "2026-11-09 15:30:00"  # 日期时间 -> 时间戳
swiss ts 2026-11-09T15:30:00+08:00  # ISO 8601
```

### 🔤 Base64

```sh
swiss b64 enc "hello"         # 编码 -> aGVsbG8=
swiss b64 dec "aGVsbG8="      # 解码 -> hello
echo hello | swiss b64 enc    # 管道输入
```

### 🌐 URL 编码

```sh
swiss url enc "你好 world"     # -> %E4%BD%A0%E5%A5%BD%20world
swiss url dec "%E4%BD%A0%E5%A5%BD"  # -> 你好
```

### 📋 JSON 美化 / 压缩

```sh
swiss json pretty '{"a":1,"b":[1,2]}'   # 美化（2 空格缩进）
swiss json compact '{"a":  1}'          # 压缩为单行
curl -s api.example.com | swiss json pretty   # 管道
```

### ✏️ 命名风格转换

```sh
swiss case camel user_name      # -> userName
swiss case snake userName       # -> user_name
swiss case kebab userName       # -> user-name
swiss case pascal user_name     # -> UserName
swiss case title user_name      # -> User Name
```

支持任意风格输入：camelCase / PascalCase / snake_case / kebab-case / 空格 / 点分隔。

### #️⃣ 哈希

```sh
swiss hash md5 "hello"          # 文本哈希
swiss hash sha256 "hello"
swiss hash sha256 -f ./file.zip # 文件哈希（流式读取，大文件不占内存）
```

### 🎲 UUID

```sh
swiss uuid                     # v4（随机）
swiss uuid v7                  # v7（时间有序）
swiss uuid -c 5                # 一次生成 5 个
```

## 开发

```sh
cargo test                     # 运行测试
cargo build --release          # 构建
```

推送 `v*.*.*` 格式的 tag 触发自动发布：

```sh
git tag v0.1.1 && git push --tags
```

CI 自动构建三个平台（macOS Apple Silicon / Windows x64 / Linux x64 glibc）的二进制，生成 SHA-256 校验和，并发布到 GitHub Release（自动生成 changelog）。

## License

MIT
