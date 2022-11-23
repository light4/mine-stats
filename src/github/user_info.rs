pub struct UserInfo;

pub const OPERATION_NAME: &str = "UserInfo";
pub const QUERY: &str = "query UserInfo($login: String!) {\n  user(login: $login) {\n    name\n    login\n    contributionsCollection {\n      totalCommitContributions\n      restrictedContributionsCount\n    }\n    repositoriesContributedTo(\n      contributionTypes: [COMMIT, ISSUE, PULL_REQUEST, REPOSITORY]\n    ) {\n      totalCount\n    }\n    pullRequests {\n      totalCount\n    }\n    openIssues: issues(states: OPEN) {\n      totalCount\n    }\n    closedIssues: issues(states: CLOSED) {\n      totalCount\n    }\n    followers {\n      totalCount\n    }\n    repositories(ownerAffiliations: OWNER) {\n      totalCount\n    }\n  }\n}\n";
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
    pub user: Option<UserInfoUser>,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUser {
    pub name: Option<String>,
    pub login: String,
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: UserInfoUserContributionsCollection,
    #[serde(rename = "repositoriesContributedTo")]
    pub repositories_contributed_to: UserInfoUserRepositoriesContributedTo,
    #[serde(rename = "pullRequests")]
    pub pull_requests: UserInfoUserPullRequests,
    #[serde(rename = "openIssues")]
    pub open_issues: UserInfoUserOpenIssues,
    #[serde(rename = "closedIssues")]
    pub closed_issues: UserInfoUserClosedIssues,
    pub followers: UserInfoUserFollowers,
    pub repositories: UserInfoUserRepositories,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserContributionsCollection {
    #[serde(rename = "totalCommitContributions")]
    pub total_commit_contributions: Int,
    #[serde(rename = "restrictedContributionsCount")]
    pub restricted_contributions_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserRepositoriesContributedTo {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserPullRequests {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserOpenIssues {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserClosedIssues {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserFollowers {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}
#[derive(Deserialize, Debug)]
pub struct UserInfoUserRepositories {
    #[serde(rename = "totalCount")]
    pub total_count: Int,
}

impl graphql_client::GraphQLQuery for UserInfo {
    type ResponseData = user_info::ResponseData;
    type Variables = user_info::Variables;

    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_info::QUERY,
            operation_name: user_info::OPERATION_NAME,
        }
    }
}
