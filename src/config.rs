use std::path::Path;

use anyhow::Result;
use kdl::KdlDocument;
use tokio::fs::read_to_string;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_stack: ListenStack,
    pub listen_port: u16,
    pub github_api_token: String,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ListenStack {
    V4,
    V6,
    Both,
}

impl Config {
    pub async fn init(path: impl AsRef<Path>) -> Result<Self> {
        let config_str = read_to_string(path).await?;
        let doc: KdlDocument = config_str.parse()?;
        let listen_stack = {
            let stack_str = doc
                .get_arg("listen_stack")
                .map(|i| i.as_string().unwrap())
                .unwrap_or("ipv4");
            match stack_str {
                "ipv4" => ListenStack::V4,
                "ipv6" => ListenStack::V6,
                "both" => ListenStack::Both,
                _ => ListenStack::V4,
            }
        };
        let r = Self {
            listen_stack,
            listen_port: doc
                .get_arg("listen_port")
                .map(|i| i.as_i64().unwrap() as u16)
                .unwrap_or(8080),
            github_api_token: doc
                .get_arg("github_api_token")
                .map(|i| i.to_string())
                .expect("must provide github api token"),
        };
        Ok(r)
    }
}
