use anyhow::Context;

use crate::client;

pub async fn list_drugs() -> Result<(), anyhow::Error> {
    let drugs = client()?.get_drugs().await.context("Failed to get defined drugs")?;
    for drug in &drugs {
        println!("{}", drug.name);
    }
    Ok(())
}
