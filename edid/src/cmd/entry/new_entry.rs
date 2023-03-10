use anyhow::Context;
use chrono::Local;
use edihkal_client::NewEntry;
use interim::{parse_date_string, Dialect};

use crate::client;

pub async fn new_entry(dose: i32, drug_name: &str, when: &str) -> Result<(), anyhow::Error> {
    // TODO: Don't assume US date dialect
    let when = parse_date_string(when, Local::now(), Dialect::Us)
        .context(format!("Failed to parse WHEN of new entry: {when}"))?;
    // TODO: Timezones
    let when_utc = when.naive_utc();

    // Look up ID of drug in edihkal using drug_name
    let drug_id = *client()?
        .get_drug_with_name(drug_name.to_string())
        .await
        .context(format!("Failed to lookup '{drug_name}' in edihkal"))?
        // TODO: Ask to define drug or do it automatically?
        // TODO: Fuzzy search drug names, it might just be a typo?
        .context(format!(
            "'{drug_name}' not found in edihkal. Typo? Try 'edid define drug {drug_name}'?"
        ))?
        .id();

    let entry = NewEntry::new(dose, drug_id, when_utc);
    client()?
        .new_entry(entry)
        .await
        .context("Failed to record new journal entry")?;

    println!("Recorded entry for {dose} {drug_name} @ {when}.");
    Ok(())
}
