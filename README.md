<div align="center">

# swiss

**瑞士军刀式开发小工具箱**

时间戳 / Base64 / URL / JSON / 命名转换 / 哈希 / UUID，一条命令搞定。

</div>

## 安装

### 从 GitHub Releases 下载

到 [Releases 页面](../../releases) 下载对应平台的压缩包，解压得到 `swiss` 二进制，放进 `PATH` 即可：

```sh
# macOS / Linux 示例
tar xzf swiss-aarch64-apple-darwin.tar.gz
sudo mv swiss /usr/local/bin/
```

或放到 `~/.local/bin`（通常已在 PATH 中）。

### 从源码构建

```sh
cargo install --path .
```

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
git tag v0.1.0 && git push --tags
```

CI 会自动构建 macOS（Apple Silicon + Intel）/ Linux（glibc + musl）/ Windows 二进制并发布到 Release。

## License

MIT
