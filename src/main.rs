use anyhow::Result;
use mine_stats::{api, config::Config};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let config_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.kdl".to_string());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "mine_stats=info,tower_http=error".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init(config_file).await.unwrap();
    info!("config: {:?}", config);

    api::run(config).await;
    Ok(())
}
