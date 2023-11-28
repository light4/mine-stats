//! contains all api services

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

use askama::Template;
use axum::{
    extract::FromRef,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tracing::info;

mod cache;
mod ip;
mod stats;
mod status;
mod themes;
mod top_langs;

use crate::{
    cache::SharedCache,
    config::{Config, ListenStack, Themes},
};

#[derive(Debug, Clone, FromRef)]
struct AppState {
    config: Config,
    themes: Themes,
    cache: SharedCache,
}

pub async fn run(config: Config, themes: Themes) {
    let localhost_v4 = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), config.listen_port);
    let localhost_v6 = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), config.listen_port);

    let listen_stack = &config.listen_stack.clone();

    let app_state = AppState {
        config,
        themes,
        cache: SharedCache::default(),
    };
    // build our application with a route
    let app = Router::new()
        .route("/api/v1/status", get(status::get_status_json))
        .route("/status", get(status::get_status))
        .route("/ip", get(ip::get_ip))
        .route("/themes", get(themes::list_themes_api))
        .route("/stats", get(stats::get_user_stats_svg))
        .route("/stats/top-langs", get(top_langs::get_top_langs_svg))
        .route("/cache/keys", get(cache::list_keys_api))
        // add a fallback service for handling routes to unknown paths
        .fallback(handler_404)
        .with_state(app_state)
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024));

    // run it
    if let ListenStack::Both = listen_stack {
        info!("listening v4 on http://{}", &localhost_v4);
        info!("listening v6 on http://{}", &localhost_v6);
        let incoming_v6 = TcpListener::bind(localhost_v6).await.unwrap();
        axum::serve(
            incoming_v6,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    } else {
        let incoming = match listen_stack {
            ListenStack::V4 => {
                info!("listening v4 on http://{}", &localhost_v4);
                TcpListener::bind(localhost_v4).await.unwrap()
            }
            ListenStack::V6 => {
                info!("listening v6 on http://{}", &localhost_v6);
                TcpListener::bind(localhost_v6).await.unwrap()
            }
            _ => unreachable!(),
        };

        axum::serve(
            incoming,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    }
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
