//! github stats api

use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::info;

use crate::{
    cache::{cache_get, cache_set, SharedCache, TIMEOUT_SECS},
    cards::form_stats_card,
    config::Config,
    github::{get_user_github_stats, stats::UserGithubStats},
};

/// get user stats from github, and return a svg
/// cache enabled
pub async fn get_user_stats_svg(
    Query(params): Query<HashMap<String, String>>,
    State(config): State<Config>,
    State(db): State<SharedCache>,
) -> Response {
    if params.get("user").is_none() {
        return (StatusCode::NOT_FOUND, "no user").into_response();
    }

    let user = params.get("user").unwrap().to_owned();

    if !config.allow_users.is_empty() && !config.allow_users.contains(&user) {
        return (StatusCode::FORBIDDEN, "user not in allow list").into_response();
    }

    let cached_stats: Option<UserGithubStats> = cache_get(&db, &user);
    let github_stats = if let Some(stats) = cached_stats {
        if stats._time.elapsed().unwrap() > TIMEOUT_SECS {
            let new_stats = get_user_github_stats(&config.github_api_token, &user).await;
            cache_set(db, &user, new_stats.clone());
            info!("update github stats cache: {}", &user);
            new_stats
        } else {
            info!("get github stats cache: {}", &user);
            stats
        }
    } else {
        let new_stats = get_user_github_stats(&config.github_api_token, &user).await;
        cache_set(db, &user, new_stats.clone());
        info!("set github stats cache: {}", &user);
        new_stats
    };
    info!("{:?}", github_stats);

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        form_stats_card(github_stats, false, true).to_string(),
    )
        .into_response()
}
