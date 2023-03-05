/// edihkal API test helpers for working with [`edihkal_client::Client`]
use edihkal_client::{Client, NewDrug};

/// Helper for defining multiple drugs by name using [`edihkal_client::Client`]
///
/// This is a shorthand for arranging tests that require multiple drugs to already be defined in the database.
pub async fn define_drugs(client: &Client, names: &Vec<&str>) {
    for name in names {
        let drug = NewDrug::new(*name);
        client.define_drug(drug).await.unwrap();
    }
}
