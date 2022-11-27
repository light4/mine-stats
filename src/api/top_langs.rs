use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};

use crate::{
    cache::{self, SharedCache},
    cards::form_top_langs_card,
    config::Config,
    github::top_langs::get_top_langs,
};

/// get user used top programming languages from github, and return a svg
/// cache enabled
pub async fn get_top_langs_svg(
    Query(params): Query<HashMap<String, String>>,
    State(config): State<Config>,
    State(db): State<SharedCache>,
) -> impl IntoResponse {
    if params.get("user").is_none() {
        return (StatusCode::NOT_FOUND, "no user").into_response();
    }

    let user = params.get("user").unwrap().to_owned();
    let hide: Vec<String> = {
        if let Some(hide) = params.get("hide") {
            hide.split(',').map(|i| i.to_string()).collect()
        } else {
            vec![]
        }
    };

    if !config.allow_users.is_empty() && !config.allow_users.contains(&user) {
        return (StatusCode::FORBIDDEN, "user not in allow list").into_response();
    }

    let data =
        cache::get_or_update(db, &user, || get_top_langs(&config.github_api_token, &user)).await;

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        form_top_langs_card(data, hide, None, None).to_string(),
    )
        .into_response()
}
