// https://github.com/openssh-rust/openssh 里面 test 有详细的例子
// 缺点是没有办法输入密码来使用
use anyhow::Result;
use openssh::{Session, KnownHosts, SessionBuilder};
use std::time::{Duration, Instant};
pub async fn openssh_demo() -> Result<()> {

    let mut sb = SessionBuilder::default();
    // 这里设置连接超时，与执行命令超时没有关系
    sb.connect_timeout(Duration::from_secs(5)).known_hosts_check(KnownHosts::Strict);

    let start = Instant::now();
    let session = sb.connect("ssh://cat@192.168.2.34:20811").await?;

    // 执行第一条命令

    // shell 相当于 command().arg()的扩展：
    // let child = session.command("ls").arg("-lht").output().await.unwrap();
    let child = session.shell("ls -lht").output().await?;
    eprintln!("{}", String::from_utf8(child.stdout).expect("server output was not valid UTF-8"));

    println!("{:?}", child.status.success()); // true
    assert!(child.stderr.is_empty()); // stderr is_empty

    // 执行第二条命令, 会一直卡在哪儿等待超时
    let child_start = Instant::now();
    let child_failed = session.shell("sleep 10").output().await.unwrap_err();
    eprintln!("child_failed: {}", child_failed);
    println!("child_start: {:?}", child_start.elapsed());
    println!("{:?}", child.status.success()); // false

    // 执行比较复杂一点的场景，比如输入命令，如果等待超时，则 kill 执行的命令
    // 执行一条命令，立即返回，它会继承父进行的 stdin, stdout, stderr
    let sleeping = session.shell("sleep 1000").spawn().await.unwrap();

    // get ID of remote ssh process
    let ppid = session
        .command("echo")
        .raw_arg("$PPID")
        .output()
        .await
        .unwrap();
    eprintln!("ppid: {:?}", ppid);
    let ppid: u32 = String::from_utf8(ppid.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // and kill it -- this kills the master connection
    let killed = session
        .command("kill")
        .arg("-9")
        .arg(&format!("{}", ppid))
        .output()
        .await
        .unwrap_err();
    eprintln!("killed: {:?}", killed); // Error::RemoteProcessTerminated

    session.close().await?;
    println!("{:?}", start.elapsed());
    Ok(())
}