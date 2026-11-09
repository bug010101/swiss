use std::io::{IsTerminal, Read};

use anyhow::{bail, Context, Result};

/// 获取输入：有参数用参数，否则读取 stdin（stdin 是终端时给出友好提示）。
pub fn input_or_stdin(arg: Option<&str>) -> Result<String> {
    match arg {
        Some(s) => Ok(s.to_string()),
        None => {
            if std::io::stdin().is_terminal() {
                bail!(
                    "缺少输入：请直接传参数，或通过管道传入，例如：\n  \
                     echo hello | swiss base64 encode"
                );
            }
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("读取 stdin 失败")?;
            Ok(buf)
        }
    }
}

/// 去掉末尾换行符（echo 管道输入会在末尾附加 \n）。
pub fn trim_eol(s: &str) -> &str {
    s.trim_end_matches(|c| c == '\n' || c == '\r')
}
