use anyhow::Result;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

use crate::cli::UrlArgs;
use crate::util::{input_or_stdin, trim_eol};

/// RFC 3986 unreserved 集合：字母数字与 `-_.~` 之外的字符全部编码。
/// 空格 -> %20，`/` -> %2F，适合编码 URL 组件与查询参数值。
const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

pub fn run(args: &UrlArgs) -> Result<()> {
    match &args.command {
        crate::cli::UrlCommand::Encode { text } => {
            let raw = input_or_stdin(text.as_deref())?;
            let input = trim_eol(&raw);
            println!("{}", utf8_percent_encode(input, ENCODE_SET));
        }
        crate::cli::UrlCommand::Decode { text } => {
            let raw = input_or_stdin(text.as_deref())?;
            let input = trim_eol(&raw);
            let decoded = percent_decode_str(input)
                .decode_utf8()
                .map_err(|e| anyhow::anyhow!("无效的 URL 编码输入: {e}"))?;
            println!("{decoded}");
        }
    }
    Ok(())
}
