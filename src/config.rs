//! app config

use std::{fmt, path::Path};

use anyhow::Result;
use kdl::KdlDocument;
use tokio::fs::read_to_string;

#[derive(Clone)]
pub struct Config {
    /// default both(ipv4 and ipv6)
    pub listen_stack: ListenStack,
    /// default 8080
    pub listen_port: u16,
    /// monitor on systemd services
    pub services: Vec<String>,
    /// use to show github stats
    pub github_api_token: String,
    /// allow query github stats user list, allow any if empty
    pub allow_users: Vec<String>,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("listen_stack", &self.listen_stack)
            .field("listen_port", &self.listen_port)
            .field("allow_users", &self.allow_users)
            .finish()
    }
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
                .unwrap_or("both");
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
            services: doc
                .get_args("services")
                .into_iter()
                .map(|i| i.as_string().unwrap().to_owned())
                .collect(),
            github_api_token: doc
                .get_arg("github_api_token")
                .and_then(|i| i.as_string())
                .map(|i| i.to_string())
                .expect("must provide github api token"),
            allow_users: doc
                .get_args("allow_users")
                .into_iter()
                .filter_map(|i| i.as_string())
                .map(|i| i.to_string())
                .collect(),
        };
        Ok(r)
    }
}
