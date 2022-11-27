use anyhow::Result;

pub mod gen;
pub mod stats;
pub mod top_langs;

pub use stats::get_user_github_stats;

const GITHUB_API: &str = "https://api.github.com/graphql";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

pub fn build_client(token: &str) -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token,)).unwrap(),
            ))
            .collect(),
        )
        .build()?;

    Ok(client)
}
