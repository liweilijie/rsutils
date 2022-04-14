use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn ssh2_demo() {
    // Connect to the ssh server
    let tcp = TcpStream::connect("222.213.23.175:22").unwrap();
    let mut sess = Session::new().unwrap();
    // 设置超时时间
    sess.set_timeout(10 * 1000);
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "123456").unwrap();
    println!("{}", sess.authenticated());

    // 创建一个 channel
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls -l").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    // 关闭连接并且等待退出状态
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());

    // channel_session不能 reused 所以需要重新创建一个
    let mut channel = sess.channel_session().unwrap();
    // 记录执行时间
    let start = std::time::Instant::now();
    // 执行等待卡住的命令
    match channel.exec("sleep 60 && echo 'hello'") {
        Ok(_) => {
            println!("ok"); // 执行成功
            let mut s = String::new();
            match channel.read_to_string(&mut s) { // 读取返回结果会失败
                Ok(_) => {
                    println!("read to string {}", s)
                },
                Err(e) => println!("read to string failed: {}", e),
            }
        },
        Err(e) => println!("exec error: {}", e),
    }

    // 关闭连接并且等待退出状态
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
    println!("elapsed: {}", start.elapsed().as_secs());
}
