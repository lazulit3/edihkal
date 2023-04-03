use anyhow::Context;
use edihkal_client::entity::drug;

use crate::client;

pub async fn define_drug<S: Into<String>>(name: S) -> Result<(), anyhow::Error> {
    client()?
        .define_drug(drug::NewModel { name: name.into() })
        .await
        .context("Failed to define drug")?;
    Ok(())
}
