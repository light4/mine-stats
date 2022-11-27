//! themes api

use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde_json::json;

use crate::config::Themes;

/// list all themes
pub async fn list_themes_api(State(themes): State<Themes>) -> impl IntoResponse {
    Json(json!({
        "items": themes.items(),
        "count": themes.len(),
        "default": themes.default(),
    }))
}
