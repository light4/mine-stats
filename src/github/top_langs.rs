use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use tracing::trace;

use super::{gen::top_langs, GITHUB_API};

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
