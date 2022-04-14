#[allow(dead_code)]
mod ssh2demo;
mod openssh_demo;

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    openssh_demo::openssh_demo().await?;

    Ok(())
}

