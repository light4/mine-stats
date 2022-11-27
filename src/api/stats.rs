//! github stats api

use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{
    cache::{self, SharedCache},
    cards::form_stats_card,
    config::Config,
    github::get_user_github_stats,
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

    let data = cache::get_or_update(db, &user, || {
        get_user_github_stats(&config.github_api_token, &user)
    })
    .await;

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        form_stats_card(data, false, true).to_string(),
    )
        .into_response()
}
