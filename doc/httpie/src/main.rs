mod cli;
mod fromstr;
use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use cli::{Commands, Get, Post};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response};

use crate::cli::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    // 生成一个 HTTP 客户端
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = match opts.subcmd {
        Commands::Get(ref args) => get(client, args).await?,
        Commands::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号+状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status)
}

// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}:{:?}", name.to_string().green(), value);
    }

    print!("\n");
}

// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        // 其它 mime type, 我们就直接输出
        _ => println!("{}", body),
    }
}

// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}
