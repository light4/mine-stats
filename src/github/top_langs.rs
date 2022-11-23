pub struct TopLang;

pub const OPERATION_NAME: &str = "TopLang";
pub const QUERY: &str = "query TopLang($login: String!) {\n  user(login: $login) {\n    # fetch only owner repos & not forks\n    repositories(ownerAffiliations: OWNER, isFork: false, first: 100) {\n      nodes {\n        name\n        languages(first: 10, orderBy: { field: SIZE, direction: DESC }) {\n          edges {\n            size\n            node {\n              color\n              name\n            }\n          }\n        }\n      }\n    }\n  }\n}\n";
use serde::{Deserialize, Serialize};

use super::*;
#[allow(dead_code)]
type Boolean = bool;
#[allow(dead_code)]
type Float = f64;
#[allow(dead_code)]
type Int = i64;
#[allow(dead_code)]
type ID = String;
#[derive(Serialize, Debug)]
pub struct Variables {
    pub login: String,
}
impl Variables {}
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub user: Option<TopLangUser>,
}
#[derive(Deserialize, Debug)]
pub struct TopLangUser {
    pub repositories: TopLangUserRepositories,
}
#[derive(Deserialize, Debug)]
pub struct TopLangUserRepositories {
    pub nodes: Option<Vec<Option<TopLangUserRepositoriesNodes>>>,
}
#[derive(Deserialize, Debug)]
pub struct TopLangUserRepositoriesNodes {
    pub name: String,
    pub languages: Option<TopLangUserRepositoriesNodesLanguages>,
}
#[derive(Deserialize, Debug)]
pub struct TopLangUserRepositoriesNodesLanguages {
    pub edges: Option<Vec<Option<TopLangUserRepositoriesNodesLanguagesEdges>>>,
}
#[derive(Deserialize, Debug)]
pub struct TopLangUserRepositoriesNodesLanguagesEdges {
    pub size: Int,
    pub node: TopLangUserRepositoriesNodesLanguagesEdgesNode,
}
#[derive(Deserialize, Debug)]
pub struct TopLangUserRepositoriesNodesLanguagesEdgesNode {
    pub color: Option<String>,
    pub name: String,
}

impl graphql_client::GraphQLQuery for TopLang {
    type ResponseData = top_langs::ResponseData;
    type Variables = top_langs::Variables;

    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: top_langs::QUERY,
            operation_name: top_langs::OPERATION_NAME,
        }
    }
}
