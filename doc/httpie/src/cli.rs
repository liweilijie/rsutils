use anyhow::{anyhow, Result};
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    pub url: String,
}

#[derive(Parser, Debug)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,
    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, Clone, PartialEq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse()方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split, 这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为key, 迭代器返回 Some(T)/None
            // 我们将其转换为 Ok(T)/Err(E), 然后用? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr, 这里可以直接s.parse()得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    // 这里我们仅仅检查一下 URL是否合法
    let _url: reqwest::Url = s.parse()?;

    Ok(s.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        )
    }
}
