use askama::Template;
use tokio::process::Command;

#[derive(Debug, Default, Template)]
#[template(path = "status.html")]
pub struct Status {
    hostname: String,
    uname: String,
    uptime: String,
    services: Vec<Service>,
}

impl Status {
    pub async fn init(service_names: Vec<String>) -> Self {
        let hostname = Command::new("hostname")
            .output()
            .await
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_else(|_| "unknown".to_string());

        let uname = Command::new("uname")
            .arg("-a")
            .output()
            .await
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_else(|_| "unkonwn".to_string());

        let uptime = Command::new("uptime")
            .output()
            .await
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_else(|_| "unknown".to_string());

        let mut services = vec![];
        for s in &service_names {
            services.push(get_service(s).await);
        }

        Self {
            hostname,
            uname,
            uptime,
            services,
        }
    }
}

#[derive(Debug)]
pub struct Service {
    name: String,
    status: ServiceStatus,
    output: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ServiceStatus {
    Active,
    Error,
    Unknown,
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Error => write!(f, "Error"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

async fn get_service(name: &str) -> Service {
    let output = Command::new("systemctl")
        .arg("status")
        .arg(name)
        .output()
        .await;

    match output {
        Ok(out) => {
            if out.status.success() {
                Service {
                    name: name.into(),
                    status: ServiceStatus::Active,
                    output: String::from_utf8_lossy(&out.stdout).into(),
                }
            } else {
                Service {
                    name: name.into(),
                    status: ServiceStatus::Error,
                    output: String::from_utf8_lossy(&out.stderr).into(),
                }
            }
        }
        Err(e) => Service {
            name: name.into(),
            status: ServiceStatus::Unknown,
            output: format!("systemctl status {} running error: {:?}", name, e),
        },
    }
}
