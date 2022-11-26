use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use askama::Template;
use axum::{
    body::Body,
    extract::{ConnectInfo, Query, State},
    http::{header, Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::server::{accept::Accept, conn::AddrIncoming};
use tracing::{info, trace};

use crate::{
    cards::form_stats_card,
    config::{Config, ListenStack},
    github,
};

pub async fn run(config: Config) {
    let localhost_v4 = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), config.listen_port);
    let incoming_v4 = AddrIncoming::bind(&localhost_v4).unwrap();

    let localhost_v6 = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), config.listen_port);
    let incoming_v6 = AddrIncoming::bind(&localhost_v6).unwrap();

    let listen_stack = &config.listen_stack.clone();

    // build our application with a route
    let app = Router::new()
        .route("/ip", get(get_ip))
        .route("/stats", get(get_user_info))
        .route("/stats/top-langs", get(get_top_langs))
        // add a fallback service for handling routes to unknown paths
        .fallback(handler_404)
        .with_state(config);

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

async fn get_ip(
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

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn get_user_info(
    Query(params): Query<HashMap<String, String>>,
    State(config): State<Config>,
) -> Response {
    dbg!(&params);
    if params.get("user").is_none() {
        return (StatusCode::NOT_FOUND, "no user").into_response();
    }

    let user = params.get("user").unwrap().to_owned();

    if !config.allow_users.is_empty() && !config.allow_users.contains(&user) {
        dbg!(&config.allow_users);
        return (StatusCode::FORBIDDEN, "user not in allow list").into_response();
    }

    let data = github::get_user_github_stats(&config.github_api_token, &user).await;

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        form_stats_card(data, false, true).to_string(),
    )
        .into_response()
}

async fn get_top_langs(
    Query(params): Query<HashMap<String, String>>,
    State(config): State<Config>,
) -> impl IntoResponse {
    if params.get("user").is_none() {
        return (StatusCode::NOT_FOUND, "nothing to see here");
    }

    let user = params.get("user").unwrap().to_owned();

    if !config.allow_users.is_empty() && !config.allow_users.contains(&user) {
        return (StatusCode::FORBIDDEN, "nothing to see here");
    }

    let client = github::build_client(&config.github_api_token).unwrap();
    let variables = github::top_langs::Variables { login: user };
    github::query_top_lang(&client, variables).await.unwrap();

    (StatusCode::OK, "get_top_langs")
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
