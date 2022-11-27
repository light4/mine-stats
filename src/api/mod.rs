//! contains all api services

use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use askama::Template;
use axum::{
    extract::FromRef,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::server::{accept::Accept, conn::AddrIncoming};
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
    let incoming_v4 = AddrIncoming::bind(&localhost_v4).unwrap();

    let localhost_v6 = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), config.listen_port);
    let incoming_v6 = AddrIncoming::bind(&localhost_v6).unwrap();

    let listen_stack = &config.listen_stack.clone();

    let app_state = AppState {
        config,
        themes,
        cache: SharedCache::default(),
    };
    // build our application with a route
    let app = Router::new()
        .route("/status", get(status::get_status))
        .route("/ip", get(ip::get_ip))
        .route("/themes", get(themes::list_themes_api))
        .route("/stats", get(stats::get_user_stats_svg))
        .route("/stats/top-langs", get(top_langs::get_top_langs_svg))
        .route("/cache/keys", get(cache::list_keys_api))
        // add a fallback service for handling routes to unknown paths
        .fallback(handler_404)
        .with_state(app_state);

    // run it
    if let ListenStack::Both = listen_stack {
        info!("listening v4 on http://{}", &localhost_v4);
        info!("listening v6 on http://{}", &localhost_v6);
        let combined = CombinedIncoming {
            a: incoming_v4,
            b: incoming_v6,
        };
        axum::Server::builder(combined)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    } else {
        let incoming = match listen_stack {
            ListenStack::V4 => {
                info!("listening v4 on http://{}", &localhost_v4);
                incoming_v4
            }
            ListenStack::V6 => {
                info!("listening v6 on http://{}", &localhost_v6);
                incoming_v6
            }
            _ => unreachable!(),
        };

        axum::Server::builder(incoming)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

struct CombinedIncoming {
    a: AddrIncoming,
    b: AddrIncoming,
}

impl Accept for CombinedIncoming {
    type Conn = <AddrIncoming as Accept>::Conn;
    type Error = <AddrIncoming as Accept>::Error;

    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        if let Poll::Ready(Some(value)) = Pin::new(&mut self.a).poll_accept(cx) {
            return Poll::Ready(Some(value));
        }

        if let Poll::Ready(Some(value)) = Pin::new(&mut self.b).poll_accept(cx) {
            return Poll::Ready(Some(value));
        }

        Poll::Pending
    }
}
