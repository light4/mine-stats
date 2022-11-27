pub struct UserRepo;

pub const OPERATION_NAME: &str = "UserRepo";
pub const QUERY: &str = "query UserRepo($login: String!, $after: String) {\n  user(login: $login) {\n    repositories(\n      first: 100\n      ownerAffiliations: OWNER\n      orderBy: { direction: DESC, field: STARGAZERS }\n      after: $after\n    ) {\n      nodes {\n        name\n        stargazers {\n          totalCount\n        }\n      }\n      pageInfo {\n        hasNextPage\n        endCursor\n      }\n    }\n  }\n}\n";
use serde::{Deserialize, Serialize};

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
    pub after: Option<String>,
}
impl Variables {}
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub user: Option<UserRepoUser>,
}
#[derive(Deserialize, Debug)]
pub struct UserRepoUser {
    pub repositories: UserRepoUserRepositories,
}
#[derive(Deserialize, Debug)]
pub struct UserRepoUserRepositories {
    pub nodes: Option<Vec<Option<UserRepoUserRepositoriesNodes>>>,
    #[serde(rename = "pageInfo")]
    pub page_info: UserRepoUserRepositoriesPageInfo,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UserRepoUserRepositoriesNodes {
    pub name: String,
    pub stargazers: UserRepoUserRepositoriesNodesStargazers,
}
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct UserRepoUserRepositoriesNodesStargazers {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserRepoUserRepositoriesPageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

impl graphql_client::GraphQLQuery for UserRepo {
    type ResponseData = ResponseData;
    type Variables = Variables;

    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
