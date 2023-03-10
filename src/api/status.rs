//! server status api

use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde_json::json;

use super::HtmlTemplate;
use crate::{config::Config, status::Status};

/// show server status: use systemd status service
pub async fn get_status(State(config): State<Config>) -> impl IntoResponse {
    let status = Status::init(config.services).await;
    HtmlTemplate(status)
}

/// show server status: use systemd status service
pub async fn get_status_json(State(config): State<Config>) -> impl IntoResponse {
    let status = Status::init(config.services).await;
    Json(json!(status))
}
