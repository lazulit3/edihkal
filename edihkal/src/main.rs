use anyhow::{Context, Result};
use axum::Server;
use edihkal_tracing::configure_tracing;
use std::net::SocketAddr;

use edihkal::{app::app, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<()> {
    configure_tracing("edihkal", "info", std::io::stdout);
    let config = get_configuration().context("Failed to read configuration")?;
    let addr: SocketAddr = format!("{}:{}", config.application.host, config.application.port)
        .parse()
        .context("Failed to parse service host and port into socket address")?;

    Server::bind(&addr)
        .serve(app(&config).await?.into_make_service())
        .await?;
    Ok(())
}
