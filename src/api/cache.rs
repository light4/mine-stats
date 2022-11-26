//! cache api

use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde_json::json;

use crate::cache::{list_keys, SharedCache};

/// list all cached kesy
pub async fn list_keys_api(State(cache): State<SharedCache>) -> impl IntoResponse {
    let keys = list_keys(&cache);
    Json(json!({
        "items": keys,
        "count": keys.len(),
    }))
}
