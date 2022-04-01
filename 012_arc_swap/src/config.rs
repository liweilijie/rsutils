use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use tokio::fs;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub network: NetworkConfig,
    pub params: ParamsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub addr: String,
    pub port: u16,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ParamsConfig {
    pub max_connections: usize,
    pub max_payload_size: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

impl From<NetworkConfig> for SocketAddr {
    fn from(n: NetworkConfig) -> Self {
        format!("{}:{}", n.addr, n.port).parse().unwrap()
    }
}

impl ServerConfig {
    pub async fn load() -> Self {
        // read yaml file from disk
        // parse yaml file to ServerConfig
        if let Ok(content) = fs::read_to_string("./fixtures/conf.yml").await {
            let config: ServerConfig = serde_yaml::from_str(&content).unwrap();
            config
        } else {
            ServerConfig::default()
        }
    }
}
