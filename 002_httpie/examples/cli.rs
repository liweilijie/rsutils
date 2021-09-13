use clap::{AppSettings, Clap};

// 定义httpie的CLI的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap会将其作为CLI的帮助
// 请参考 https://github.com/clap-rs/clap

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "li wei <liwei@ypool.io>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不用的HTTP方法，目前只支持get/post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂时不支持其它http方法
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    url: String
}

// post 子命令。 需要输入一个url和若干个可选的 key=value 用于提供json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Clap, Debug)]
struct Post {
    /// HTTP 请求的URL
    url: String,
    /// HTTP 请求的body
    body: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}