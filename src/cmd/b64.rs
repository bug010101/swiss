use anyhow::{bail, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::cli::Base64Args;
use crate::util::{input_or_stdin, trim_eol};

pub fn run(args: &Base64Args) -> Result<()> {
    match &args.command {
        crate::cli::Base64Command::Encode { text } => {
            let input = input_or_stdin(text.as_deref())?;
            println!("{}", STANDARD.encode(input.as_bytes()));
        }
        crate::cli::Base64Command::Decode { text } => {
            let raw = input_or_stdin(text.as_deref())?;
            let input = trim_eol(&raw);
            // 容忍换行/空白（多行 Base64 很常见）
            let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
            let bytes = STANDARD
                .decode(cleaned.as_bytes())
                .map_err(|e| anyhow::anyhow!("无效的 Base64 输入: {e}"))?;
            match String::from_utf8(bytes) {
                Ok(s) => println!("{s}"),
                Err(_) => bail!("解码结果不是合法 UTF-8 文本（可能是二进制数据）"),
            }
        }
    }
    Ok(())
}
