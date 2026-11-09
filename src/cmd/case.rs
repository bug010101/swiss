use anyhow::Result;

use crate::cli::{CaseArgs, CaseStyle};
use crate::util::{input_or_stdin, trim_eol};

pub fn run(args: &CaseArgs) -> Result<()> {
    let raw = input_or_stdin(args.input.as_deref())?;
    let input = trim_eol(&raw);
    println!("{}", convert(args.style, input));
    Ok(())
}

pub fn convert(style: CaseStyle, input: &str) -> String {
    let words = split_words(input);
    match style {
        CaseStyle::Camel => words
            .iter()
            .enumerate()
            .map(|(i, w)| if i == 0 { w.to_lowercase() } else { capitalize(w) })
            .collect::<Vec<_>>()
            .join(""),
        CaseStyle::Pascal => words.iter().map(|w| capitalize(w)).collect::<Vec<_>>().join(""),
        CaseStyle::Snake => words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>().join("_"),
        CaseStyle::Kebab => words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>().join("-"),
        CaseStyle::Title => words.iter().map(|w| capitalize(w)).collect::<Vec<_>>().join(" "),
    }
}

/// 把任意命名风格拆成单词列表。
/// 支持 camelCase / PascalCase / snake_case / kebab-case / 空格 / 点分隔，
/// 连续大写缩写词（HTTP、URL、ID）也能正确处理。
fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut prev_is_lower_or_digit = false;

    for ch in input.chars() {
        match ch {
            '_' | '-' | ' ' | '.' => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
                prev_is_lower_or_digit = false;
            }
            c if c.is_uppercase() => {
                if prev_is_lower_or_digit && !current.is_empty() {
                    // lower -> Upper 边界：getUserName -> [get, User, Name]
                    words.push(std::mem::take(&mut current));
                }
                current.push(c);
                prev_is_lower_or_digit = false;
            }
            c => {
                // Upper -> lower 边界：缩写词结束，最后一个大写属于新词。
                // getHTTPResponse -> [get, HTTP, Response]
                if current.chars().nth_back(1).map_or(false, |p| p.is_uppercase())
                    && current.chars().last().map_or(false, |l| l.is_uppercase())
                {
                    if let Some(last) = current.pop() {
                        if !current.is_empty() {
                            words.push(std::mem::take(&mut current));
                        }
                        current.push(last);
                    }
                }
                current.push(c);
                prev_is_lower_or_digit = c.is_lowercase() || c.is_ascii_digit();
            }
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    if words.is_empty() {
        vec![input.to_string()]
    } else {
        words
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::convert;
    use crate::cli::CaseStyle;
    use CaseStyle::*;

    fn c(style: CaseStyle, input: &str) -> String {
        convert(style, input)
    }

    #[test]
    fn snake_to_camel() {
        assert_eq!(c(Camel, "user_name"), "userName");
        assert_eq!(c(Camel, "get_http_response"), "getHttpResponse");
    }

    #[test]
    fn camel_to_snake() {
        assert_eq!(c(Snake, "userName"), "user_name");
        assert_eq!(c(Snake, "getHTTPResponse"), "get_http_response");
    }

    #[test]
    fn kebab_roundtrip() {
        assert_eq!(c(Kebab, "user_name"), "user-name");
        assert_eq!(c(Kebab, "userName"), "user-name");
        assert_eq!(c(Snake, "user-name"), "user_name");
    }

    #[test]
    fn pascal_and_title() {
        assert_eq!(c(Pascal, "user_name"), "UserName");
        assert_eq!(c(Title, "user_name"), "User Name");
    }

    #[test]
    fn space_separated() {
        assert_eq!(c(Snake, "hello world"), "hello_world");
        assert_eq!(c(Camel, "hello world foo"), "helloWorldFoo");
    }

    #[test]
    fn single_word() {
        assert_eq!(c(Snake, "hello"), "hello");
        assert_eq!(c(Pascal, "hello"), "Hello");
        assert_eq!(c(Camel, "HELLO"), "hello");
    }

    #[test]
    fn dotted_names() {
        assert_eq!(c(Snake, "my.app.config"), "my_app_config");
    }

    #[test]
    fn with_digits() {
        assert_eq!(c(Snake, "user2Name"), "user2_name");
        assert_eq!(c(Camel, "v2_parser"), "v2Parser");
    }
}
