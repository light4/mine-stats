use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use tracing::info;

pub mod top_langs;
pub mod user_info;

use top_langs::TopLang;
use user_info::UserInfo;

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

pub async fn query_user_info(
    client: &Client,
    variables: user_info::Variables,
) -> Result<user_info::ResponseData> {
    let request_body = UserInfo::build_query(variables);
    let res = client.post(GITHUB_API).json(&request_body).send().await?;
    let response_body: Response<user_info::ResponseData> = res.json().await?;
    info!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}

pub async fn query_top_lang(
    client: &Client,
    variables: top_langs::Variables,
) -> Result<top_langs::ResponseData> {
    let request_body = TopLang::build_query(variables);
    let res = client.post(GITHUB_API).json(&request_body).send().await?;
    let response_body: Response<top_langs::ResponseData> = res.json().await?;
    info!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}

#[derive(Debug, Clone)]
pub struct UserGithubStats {
    pub login: String,
    pub name: String,
    pub stars: i64,
    pub commits: i64,
    pub prs: i64,
    pub issues: i64,
    pub contribs: i64,
}

impl From<user_info::ResponseData> for UserGithubStats {
    fn from(info: user_info::ResponseData) -> Self {
        let user = info.user.unwrap();
        Self {
            login: user.login,
            name: user.name.unwrap_or_default(),
            stars: user.followers.total_count,
            commits: user.contributions_collection.total_commit_contributions,
            prs: user.pull_requests.total_count,
            issues: user.open_issues.total_count + user.closed_issues.total_count,
            contribs: user.repositories_contributed_to.total_count,
        }
    }
}
