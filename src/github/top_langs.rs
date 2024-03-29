use std::collections::HashMap;

use bincode::{Decode, Encode};
use color_eyre::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use tracing::trace;

use super::{build_client, gen::top_langs, GITHUB_API};
use crate::utils::{MonitorTime, SystemTimeWrapper};

pub async fn query_top_langs(
    client: &Client,
    variables: top_langs::Variables,
) -> Result<top_langs::ResponseData> {
    let request_body = top_langs::TopLang::build_query(variables);
    let res = client.post(GITHUB_API).json(&request_body).send().await?;
    let response_body: Response<top_langs::ResponseData> = res.json().await?;
    trace!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct Lang {
    pub name: String,
    pub color: Option<String>,
    pub size: usize,
}

#[derive(Debug, Clone, Default, Decode, Encode)]
pub struct TopLangs {
    pub langs: HashMap<String, Lang>,
    pub(crate) __create_at: SystemTimeWrapper,
}

impl MonitorTime for TopLangs {
    fn create_at(&self) -> SystemTimeWrapper {
        self.__create_at
    }
}

pub async fn get_top_langs(token: &str, username: &str) -> TopLangs {
    let mut langs_map = HashMap::new();
    let client = build_client(token).unwrap();
    let variables = top_langs::Variables {
        login: username.to_string(),
    };
    let data = query_top_langs(&client, variables).await.unwrap();
    let nodes = data.user.unwrap().repositories.nodes;
    if let Some(inner_nodes) = nodes {
        for lang in inner_nodes.into_iter().flatten() {
            if let Some(inner_inner_lang) = lang.languages {
                if let Some(edges) = inner_inner_lang.edges {
                    for edge in edges.into_iter().flatten() {
                        let name = edge.node.name.to_string();
                        let item = Lang {
                            name: edge.node.name.to_string(),
                            color: edge.node.color,
                            size: edge.size as usize,
                        };
                        if langs_map.get(&name).is_none() {
                            langs_map.insert(name, item);
                        } else {
                            let origin = langs_map.get_mut(&name).unwrap();
                            origin.size += item.size;
                        }
                    }
                }
            }
        }
    }

    TopLangs {
        langs: langs_map,
        __create_at: SystemTimeWrapper::default(),
    }
}
