use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use tracing::trace;

pub mod stats;
pub mod top_langs;
pub mod user_info;
pub mod user_repos;

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

pub async fn query_user_info(
    client: &Client,
    variables: user_info::Variables,
) -> Result<user_info::ResponseData> {
    let request_body = user_info::UserInfo::build_query(variables);
    let res = client.post(GITHUB_API).json(&request_body).send().await?;
    let response_body: Response<user_info::ResponseData> = res.json().await?;
    trace!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}

pub async fn query_top_lang(
    client: &Client,
    variables: top_langs::Variables,
) -> Result<top_langs::ResponseData> {
    let request_body = top_langs::TopLang::build_query(variables);
    let res = client.post(GITHUB_API).json(&request_body).send().await?;
    let response_body: Response<top_langs::ResponseData> = res.json().await?;
    trace!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}

pub async fn query_user_repos(
    client: &Client,
    variables: user_repos::Variables,
) -> Result<user_repos::ResponseData> {
    let request_body = user_repos::UserRepo::build_query(variables);
    let res = client.post(GITHUB_API).json(&request_body).send().await?;
    let response_body: Response<user_repos::ResponseData> = res.json().await?;
    trace!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}
