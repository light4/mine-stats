use color_eyre::Result;
use mine_stats::{
    api,
    config::{Config, Themes},
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config_path = std::env::args().nth(1).unwrap_or_default();
    let config_file = format!("{config_path}config.kdl");
    let themes_file = format!("{config_path}themes.kdl");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "mine_stats=info,tower_http=error".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init(config_file).await.unwrap();
    info!("config: {:?}", config);
    let themes = Themes::init(themes_file).await.unwrap_or_default();

    api::run(config, themes).await;
    Ok(())
}
