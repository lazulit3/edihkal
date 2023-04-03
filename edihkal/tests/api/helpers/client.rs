/// edihkal API test helpers for working with [`edihkal_client::Client`]
use edihkal_client::{entity::drug, Client};

/// Helper for defining multiple drugs by name using [`edihkal_client::Client`]
///
/// This is a shorthand for arranging tests that require multiple drugs to already be defined in the database.
pub async fn define_drugs(client: &Client, names: &Vec<&str>) {
    for name in names {
        let drug = drug::NewModel::new(name.to_string());
        client.define_drug(drug).await.unwrap();
    }
}
