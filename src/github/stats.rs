use anyhow::Result;
use bincode::{Decode, Encode};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use tracing::trace;

use super::{
    build_client,
    gen::{user_info, user_repos},
    GITHUB_API,
};
use crate::utils::{MonitorTime, SystemTimeWrapper};

#[derive(Debug, Clone, Default, Decode, Encode)]
pub struct UserGithubStats {
    pub login: String,
    pub name: String,
    pub stars: i64,
    pub commits: i64,
    pub repos: i64,
    pub prs: i64,
    pub issues: i64,
    pub contribs: i64,
    pub followers: i64,
    pub rank: Rank,
    pub(crate) __create_at: SystemTimeWrapper,
}

impl MonitorTime for UserGithubStats {
    fn create_at(&self) -> SystemTimeWrapper {
        self.__create_at
    }
}

impl UserGithubStats {
    fn calculate_rank(&self) -> Rank {
        const COMMITS_OFFSET: f64 = 1.65;
        const CONTRIBS_OFFSET: f64 = 1.65;
        const ISSUES_OFFSET: f64 = 1.;
        const STARS_OFFSET: f64 = 0.75;
        const PRS_OFFSET: f64 = 0.5;
        const FOLLOWERS_OFFSET: f64 = 0.45;
        const REPO_OFFSET: f64 = 1.;

        const ALL_OFFSETS: f64 = CONTRIBS_OFFSET
            + ISSUES_OFFSET
            + STARS_OFFSET
            + PRS_OFFSET
            + FOLLOWERS_OFFSET
            + REPO_OFFSET;

        const RANK_S_VALUE: u8 = 1;
        const RANK_DOUBLE_A_VALUE: u8 = 25;
        const RANK_A2_VALUE: u8 = 45;
        const RANK_A3_VALUE: u8 = 60;
        const RANK_B_VALUE: u8 = 100;

        const TOTAL_VALUES: u8 =
            RANK_S_VALUE + RANK_DOUBLE_A_VALUE + RANK_A2_VALUE + RANK_A3_VALUE + RANK_B_VALUE;

        let score = (self.commits as f64 * COMMITS_OFFSET
            + self.contribs as f64 * CONTRIBS_OFFSET
            + self.issues as f64 * ISSUES_OFFSET
            + self.stars as f64 * STARS_OFFSET
            + self.prs as f64 * PRS_OFFSET
            + self.followers as f64 * FOLLOWERS_OFFSET
            + self.repos as f64 * REPO_OFFSET)
            / 100.;
        let normalized_score =
            (normalcdf(score, TOTAL_VALUES as f64, ALL_OFFSETS) * 100.).round() as u8;

        let level = match normalized_score {
            n if n < RANK_S_VALUE => "S+",
            n if n < RANK_DOUBLE_A_VALUE => "S",
            n if n < RANK_A2_VALUE => "A++",
            n if n < RANK_A3_VALUE => "A+",
            _ => "B+",
        };

        Rank {
            level: level.to_owned(),
            score: normalized_score,
        }
    }

    pub fn update_rank(&mut self) {
        self.rank = self.calculate_rank()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
pub struct Rank {
    pub level: String,
    pub score: u8,
}

impl Default for Rank {
    fn default() -> Self {
        Self {
            level: "C".to_string(),
            score: 0,
        }
    }
}

fn normalcdf(mean: f64, sigma: f64, to: f64) -> f64 {
    let z = (to - mean) / (2. * sigma * sigma).sqrt();
    let t = 1. / (1. + 0.3275911 * z.abs());
    let a1 = 0.254_829_592;
    let a2 = -0.284_496_736;
    let a3 = 1.421_413_741;
    let a4 = -1.453_152_027;
    let a5 = 1.061_405_429;
    let erf = 1. - ((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t * (-z * z).exp();

    (1. / 2.) * (1. + z.signum() * erf)
}

async fn fetch_total_stars(token: &str, user: &str, repo_to_hide: Vec<String>) -> i64 {
    let client = build_client(token).unwrap();

    let mut nodes = vec![];
    let mut has_next_page = true;
    let mut end_cursor = None;
    while has_next_page {
        let variables = user_repos::Variables {
            login: user.to_string(),
            after: end_cursor,
        };
        let res = query_user_repos(&client, variables).await.unwrap();
        let repos = res.user.unwrap().repositories;

        if let Some(inner_nodes) = repos.nodes {
            for inner_node in inner_nodes {
                if inner_node.is_some() {
                    let real_node = inner_node.unwrap();
                    if real_node.stargazers.total_count as usize > 0 {
                        nodes.push(real_node)
                    }
                }
            }
        }
        has_next_page = repos.page_info.has_next_page;
        end_cursor = repos.page_info.end_cursor;
    }

    nodes
        .iter()
        .filter(|i| !repo_to_hide.contains(&i.name))
        .map(|i| i.stargazers.total_count)
        .sum()
}

pub async fn get_user_github_stats(token: &str, username: &str) -> UserGithubStats {
    let client = build_client(token).unwrap();
    let variables = user_info::Variables {
        login: username.to_string(),
    };
    let data = query_user_info(&client, variables).await.unwrap();
    let user = data.user.unwrap();

    let stars = fetch_total_stars(token, username, vec![]).await;
    trace!("total_stars: {}", stars);

    let mut stats = UserGithubStats {
        login: user.login.clone(),
        name: user.name.unwrap_or(user.login.clone()),
        stars,
        commits: user.contributions_collection.total_commit_contributions,
        repos: user.repositories.total_count,
        prs: user.pull_requests.total_count,
        issues: user.open_issues.total_count + user.closed_issues.total_count,
        contribs: user.repositories_contributed_to.total_count,
        followers: user.followers.total_count,
        rank: Rank::default(),
        __create_at: SystemTimeWrapper::default(),
    };
    stats.update_rank();
    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rank() {
        let stats = UserGithubStats {
            commits: 100,
            repos: 5,
            followers: 100,
            contribs: 61,
            stars: 400,
            prs: 300,
            issues: 200,
            ..Default::default()
        };
        let rank = stats.calculate_rank();
        assert_eq!(
            rank,
            Rank {
                level: "A+".to_string(),
                score: 49
            }
        );
    }
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
