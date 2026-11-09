use anyhow::{bail, Context, Result};
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

use crate::cli::TimestampArgs;
use crate::util::trim_eol;

pub fn run(args: &TimestampArgs) -> Result<()> {
    let input = match &args.input {
        Some(s) => trim_eol(s),
        None => {
            print_now()?;
            return Ok(());
        }
    };

    if input.is_empty() {
        bail!("输入为空");
    }

    if let Ok(ts) = input.parse::<i64>() {
        print_from_timestamp(ts)?;
    } else {
        print_from_datetime(input)?;
    }
    Ok(())
}

/// 打印当前时间的各种表示。
fn print_now() -> Result<()> {
    let local = Local::now();
    let utc = Utc::now();
    println!("本地   {}", local.format("%Y-%m-%d %H:%M:%S %z"));
    println!("UTC    {}", utc.format("%Y-%m-%d %H:%M:%S"));
    println!("秒级   {}", utc.timestamp());
    println!("毫秒   {}", utc.timestamp_millis());
    Ok(())
}

/// 秒级或毫秒级时间戳 -> 日期时间。
fn print_from_timestamp(ts: i64) -> Result<()> {
    // 13 位量级视为毫秒，10 位量级视为秒
    let (secs, millis) = if ts.abs() >= 100_000_000_000 {
        (ts.div_euclid(1000), ts.rem_euclid(1000))
    } else {
        (ts, 0)
    };
    let dt_utc = Utc
        .timestamp_opt(secs, 0)
        .single()
        .context("无效的时间戳")?;
    let dt_local: DateTime<Local> = dt_utc.into();
    println!("本地   {}", dt_local.format("%Y-%m-%d %H:%M:%S"));
    println!("UTC    {}", dt_utc.format("%Y-%m-%d %H:%M:%S"));
    if millis != 0 {
        println!("毫秒   {}", dt_utc.timestamp_millis());
    }
    Ok(())
}

/// 日期时间字符串 -> 时间戳，先试 ISO 8601，再试常见格式。
fn print_from_datetime(input: &str) -> Result<()> {
    let parsed = DateTime::parse_from_rfc3339(input).ok().or_else(|| try_common_formats(input));

    match parsed {
        Some(dt) => {
            let dt_utc: DateTime<Utc> = dt.into();
            let dt_local: DateTime<Local> = dt.into();
            println!("秒级   {}", dt_utc.timestamp());
            println!("毫秒   {}", dt_utc.timestamp_millis());
            println!("本地   {}", dt_local.format("%Y-%m-%d %H:%M:%S"));
            println!("UTC    {}", dt_utc.format("%Y-%m-%d %H:%M:%S"));
            Ok(())
        }
        None => bail!(
            "无法识别的时间格式: {input}\n支持的例子：2026-11-09 15:30:00 / 2026-11-09T15:30:00+08:00 / 1762671000"
        ),
    }
}

/// 尝试常见日期时间格式（按本地时区解释）。
fn try_common_formats(input: &str) -> Option<DateTime<FixedOffset>> {
    const DATETIME_FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%Y-%m-%d %H:%M",
    ];
    const DATE_FORMATS: &[&str] = &["%Y-%m-%d", "%Y/%m/%d"];

    for fmt in DATETIME_FORMATS {
        if let Ok(naive) = NaiveDateTime::parse_from_str(input, fmt) {
            if let Some(local) = Local.from_local_datetime(&naive).single() {
                return Some(local.fixed_offset());
            }
        }
    }
    for fmt in DATE_FORMATS {
        if let Ok(date) = NaiveDate::parse_from_str(input, fmt) {
            let naive = date.and_hms_opt(0, 0, 0)?;
            if let Some(local) = Local.from_local_datetime(&naive).single() {
                return Some(local.fixed_offset());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_with_offset() {
        // 2026-11-09 15:30:00 +08:00 == 07:30 UTC
        let dt = DateTime::parse_from_rfc3339("2026-11-09T15:30:00+08:00").unwrap();
        assert_eq!(dt.timestamp(), 1_794_209_400);
        let dt = DateTime::parse_from_rfc3339("2026-11-09T07:30:00Z").unwrap();
        assert_eq!(dt.timestamp(), 1_794_209_400);
    }

    #[test]
    fn epoch_zero() {
        let dt = DateTime::parse_from_rfc3339("1970-01-01T00:00:00+00:00").unwrap();
        assert_eq!(dt.timestamp(), 0);
    }

    #[test]
    fn common_format_matches() {
        assert!(try_common_formats("2026-11-09 15:30:00").is_some());
        assert!(try_common_formats("2026/11/09").is_some());
        assert!(try_common_formats("2026-11-09").is_some());
        assert!(try_common_formats("not a date").is_none());
    }
}
