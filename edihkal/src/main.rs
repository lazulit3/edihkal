use anyhow::{Context, Result};
use axum::Server;
use std::net::SocketAddr;

use edihkal::{app::app, configuration::get_configuration, tracing::configure_tracing};

#[tokio::main]
async fn main() -> Result<()> {
    configure_tracing("edihkal", "info");
    let configuration = get_configuration().context("Failed to read configuration")?;
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));

    Server::bind(&addr)
        .serve(app(&configuration).await.into_make_service())
        .await?;
    Ok(())
}
