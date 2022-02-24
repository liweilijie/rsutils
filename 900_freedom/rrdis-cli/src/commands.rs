use bytes::{BufMut, BytesMut};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub enum Commands {
    /// push value to list
    Rpush {
        key: String,
        /// value
        values: Vec<String>,
    },
}

impl Commands {
    pub fn to_bytes(&self) -> bytes::BytesMut {
        let cmd = match self {
            Commands::Rpush { key, values } => {
                let mut builder = CmdBuilder::new().arg("RPUSH").arg(key);
                values.iter().for_each(|v| builder.add_arg(v));
                builder.to_bytes()
            }
        };
        cmd
    }
}

#[derive(Debug, Clone)]
struct CmdBuilder {
    args: Vec<String>,
}

impl CmdBuilder {
    pub fn new() -> Self {
        CmdBuilder { args: vec![] }
    }

    fn arg(mut self, arg: &str) -> Self {
        self.args.push(format!("${}", arg.len()));
        self.args.push(arg.to_string());
        self
    }

    fn add_arg(&mut self, arg: &str) {
        self.args.push(format!("${}", arg.len()));
        self.args.push(arg.to_string());
    }

    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.put(&format!("*{}\r\n", self.args.len() / 2).into_bytes()[..]);
        bytes.put(&self.args.join("\r\n").into_bytes()[..]);
        bytes.put(&b"\r\n"[..]);
        bytes
    }
}
