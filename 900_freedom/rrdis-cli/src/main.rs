#![allow(dead_code)]

use bytes::{BufMut, BytesMut};
use std::error::Error;
use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio::prelude::*;

mod commands;
mod reply;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let com = commands::Commands::from_args();
    let mut stream = TcpStream::connect("222.213.23.209:6379").await?;
    let mut buf = [0u8; 1024];
    let mut resp = BytesMut::with_capacity(1024);

    let (mut reader, mut writer) = stream.split();
    // 向服务器发送 PING
    writer.write(&com.to_bytes()).await?;
    let n = reader.read(&mut buf).await?;
    resp.put(&buf[0..n]);
    // 返回结果应该是 PONG
    println!("{:?}", resp);
    Ok(())
}
