use anyhow::Context;
use edihkal_client::NewDrug;

use crate::client;

pub async fn define_drug(name: &str) -> Result<(), anyhow::Error> {
    let drug = client()?
        .define_drug(NewDrug::new(name))
        .await
        .context("Failed to define drug")?;
    println!("{} has been defined.", drug.name());
    Ok(())
}
