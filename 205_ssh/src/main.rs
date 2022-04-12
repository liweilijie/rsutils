use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    // Connect to the ssh server
    let tcp = TcpStream::connect("222.213.23.175:22").unwrap();
    let mut sess = Session::new().unwrap();
    // 设置超时时间
    sess.set_timeout(20 * 1000);
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "123456").unwrap();
    println!("{}", sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls -l").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}
