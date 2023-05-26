use anyhow::{Context, Result};
use axum_server::tls_rustls::RustlsConfig;
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

    let tls_config = RustlsConfig::from_pem_file(&config.tls.certificate, &config.tls.key)
        .await
        .context("Failed to configure rustls using configured certificate and key")?;

    axum_server::bind_rustls(addr, tls_config)
        .serve(app(&config).await?.into_make_service())
        .await?;
    Ok(())
}
