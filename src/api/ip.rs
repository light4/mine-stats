//! ip api

use std::net::SocketAddr;

use axum::{body::Body, extract::ConnectInfo, http::Request, response::IntoResponse};
use tracing::trace;

/// return request client ip
pub async fn get_ip(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
) -> impl IntoResponse {
    trace!(?req);
    trace!(?addr);
    let req_ip_str = addr.ip().to_string();
    let ip = req
        .headers()
        .get("X-Forwarded-For")
        .map(|i| i.to_str().unwrap())
        .unwrap_or(&req_ip_str);
    ip.to_string()
}
