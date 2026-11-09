use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::cmd;

#[derive(Parser)]
#[command(
    name = "swiss",
    version,
    about = "瑞士军刀式开发小工具箱：时间戳 / Base64 / URL / JSON / 命名转换 / 哈希 / UUID"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Timestamp(args) => cmd::timestamp::run(&args),
            Command::Base64(args) => cmd::b64::run(&args),
            Command::Url(args) => cmd::url::run(&args),
            Command::Json(args) => cmd::json::run(&args),
            Command::Case(args) => cmd::case::run(&args),
            Command::Hash(args) => cmd::hash::run(&args),
            Command::Uuid(args) => cmd::uuid::run(&args),
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// 当前时间 / 时间戳与日期互转
    #[command(alias = "ts")]
    Timestamp(TimestampArgs),
    /// Base64 编码与解码
    #[command(alias = "b64")]
    Base64(Base64Args),
    /// URL 百分号编码与解码
    Url(UrlArgs),
    /// JSON 美化与压缩
    Json(JsonArgs),
    /// 命名风格转换（camel / snake / kebab / ...）
    Case(CaseArgs),
    /// 计算文本或文件的哈希（md5 / sha1 / sha256 / sha512）
    Hash(HashArgs),
    /// 生成 UUID（v4 / v7）
    Uuid(UuidArgs),
}

#[derive(Args)]
pub struct TimestampArgs {
    /// 时间戳或时间字符串；省略则显示当前时间
    pub input: Option<String>,
}

#[derive(Args)]
pub struct Base64Args {
    #[command(subcommand)]
    pub command: Base64Command,
}

#[derive(Subcommand)]
pub enum Base64Command {
    /// 编码为 Base64
    #[command(alias = "enc")]
    Encode {
        /// 要编码的文本；省略则读取 stdin
        text: Option<String>,
    },
    /// 解码 Base64
    #[command(alias = "dec")]
    Decode {
        /// 要解码的文本；省略则读取 stdin
        text: Option<String>,
    },
}

#[derive(Args)]
pub struct UrlArgs {
    #[command(subcommand)]
    pub command: UrlCommand,
}

#[derive(Subcommand)]
pub enum UrlCommand {
    /// 编码为 URL 百分号格式
    #[command(alias = "enc")]
    Encode {
        /// 要编码的文本；省略则读取 stdin
        text: Option<String>,
    },
    /// 解码 URL 百分号格式
    #[command(alias = "dec")]
    Decode {
        /// 要解码的文本；省略则读取 stdin
        text: Option<String>,
    },
}

#[derive(Args)]
pub struct JsonArgs {
    #[command(subcommand)]
    pub command: JsonCommand,
}

#[derive(Subcommand)]
pub enum JsonCommand {
    /// 美化 JSON（2 空格缩进）
    Pretty {
        /// JSON 文本；省略则读取 stdin
        json: Option<String>,
    },
    /// 压缩 JSON 为单行
    Compact {
        /// JSON 文本；省略则读取 stdin
        json: Option<String>,
    },
}

#[derive(Args)]
pub struct CaseArgs {
    /// 目标命名风格
    pub style: CaseStyle,
    /// 输入文本；省略则读取 stdin
    pub input: Option<String>,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum CaseStyle {
    Camel,
    Pascal,
    Snake,
    Kebab,
    Title,
}

#[derive(Args)]
pub struct HashArgs {
    /// 哈希算法
    pub algorithm: HashAlgorithm,
    /// 输入文本；省略则读取 stdin
    pub text: Option<String>,
    /// 改为对文件计算哈希
    #[arg(short, long, conflicts_with = "text")]
    pub file: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Args)]
pub struct UuidArgs {
    /// UUID 版本
    #[arg(default_value = "v4")]
    pub version: UuidVersion,
    /// 生成数量
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum UuidVersion {
    V4,
    V7,
}
