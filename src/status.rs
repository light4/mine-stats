use std::time::Duration;

use askama::Template;
use color_eyre::Result;
use serde::Serialize;
use tokio::process::Command;

use crate::humantime::HumanTime;

#[derive(Debug, Default, Serialize)]
pub struct PkgInfo {
    name: &'static str,
    version: &'static str,
    homepage: &'static str,
}

#[derive(Debug, Default, Template, Serialize)]
#[template(path = "status.html")]
pub struct Status {
    utsname: MyUtsName,
    sysinfo: MySysInfo,
    services: Vec<Service>,
    pkginfo: PkgInfo,
}

#[derive(Debug, Default, Serialize)]
pub struct MyUtsName {
    sysname: String,
    nodename: String,
    release: String,
    version: String,
    machine: String,
    domainname: String,
}

impl MyUtsName {
    pub fn init() -> Result<Self> {
        let output = nix::sys::utsname::uname()?;
        Ok(Self {
            sysname: output.sysname().to_string_lossy().to_string(),
            nodename: output.nodename().to_string_lossy().to_string(),
            release: output.release().to_string_lossy().to_string(),
            version: output.version().to_string_lossy().to_string(),
            machine: output.machine().to_string_lossy().to_string(),
            domainname: output.domainname().to_string_lossy().to_string(),
        })
    }

    pub fn as_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.sysname, self.nodename, self.release, self.version, self.machine, self.domainname
        )
    }
}

#[derive(Debug, Default, Serialize)]
pub struct MySysInfo {
    load_average: (f64, f64, f64),
    uptime: Duration,
    process_count: u16,
    swap_total: u64,
    swap_free: u64,
    ram_total: u64,
    ram_unused: u64,
}

impl MySysInfo {
    pub fn init() -> Result<Self> {
        let output = nix::sys::sysinfo::sysinfo()?;
        Ok(Self {
            load_average: output.load_average(),
            uptime: output.uptime(),
            process_count: output.process_count(),
            swap_total: output.swap_total(),
            swap_free: output.swap_free(),
            ram_total: output.ram_total(),
            ram_unused: output.ram_unused(),
        })
    }

    pub fn uptime_humanize(&self) -> String {
        let ht = HumanTime::from(self.uptime);
        format!("{:#}", ht)
    }

    pub fn load_average_string(&self) -> String {
        format!(
            "{:.2} {:.2} {:.2}",
            self.load_average.0, self.load_average.1, self.load_average.2
        )
    }
}

impl Status {
    pub async fn init(service_names: Vec<String>) -> Self {
        let utsname = MyUtsName::init().unwrap_or_default();
        let sysinfo = MySysInfo::init().unwrap_or_default();

        let mut services = vec![];
        for s in &service_names {
            services.push(get_service(s).await);
        }

        let pkginfo = PkgInfo {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            homepage: env!("CARGO_PKG_HOMEPAGE"),
        };
        Self {
            utsname,
            sysinfo,
            services,
            pkginfo,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Service {
    name: String,
    status: ServiceStatus,
    output: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
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
            output: format!("systemctl status {name} running error: {e:?}"),
        },
    }
}
