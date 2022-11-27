use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    cache::{cache_get, cache_set, SharedCache, TIMEOUT_SECS},
    cards::form_top_langs_card,
    config::Config,
    github::top_langs::{get_top_langs, TopLangs},
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
            hide.split(",").map(|i| i.to_string()).collect()
        } else {
            vec![]
        }
    };

    if !config.allow_users.is_empty() && !config.allow_users.contains(&user) {
        return (StatusCode::FORBIDDEN, "user not in allow list").into_response();
    }

    let cached_data: Option<TopLangs> = cache_get(&db, &user);
    let data = if let Some(d) = cached_data {
        if d._time.elapsed().unwrap() > TIMEOUT_SECS {
            let new_data = get_top_langs(&config.github_api_token, &user).await;
            cache_set(db, &user, new_data.clone());
            info!("update top_langs cache: {}", &user);
            new_data
        } else {
            info!("get top_langs cache: {}", &user);
            d
        }
    } else {
        let new_data = get_top_langs(&config.github_api_token, &user).await;
        cache_set(db, &user, new_data.clone());
        info!("set top_langs cache: {}", &user);
        new_data
    };
    info!("{:?}", data);

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        form_top_langs_card(data, hide, None, None).to_string(),
    )
        .into_response()
}
