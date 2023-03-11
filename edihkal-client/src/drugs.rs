use std::collections::HashMap;

// Re-export drug Model as Drug for client-side apps.
use entity::drug::Model as Drug;
use entity::NewDrug;

use crate::{
    edihkal::{Client, Filters, Payloads},
    errors::Error,
};

pub(crate) struct NewDrugEndpoint;
impl Payloads for NewDrugEndpoint {
    type Request = NewDrug;
    type Response = Drug;
}

pub(crate) struct DrugsEndpoint;
impl Payloads for DrugsEndpoint {
    type Request = ();
    type Response = Vec<Drug>;
}

impl Client {
    /// Define a drug in edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn define_drug(&self, drug: NewDrug) -> Result<Drug, Error> {
        self.post::<NewDrugEndpoint>("/drugs", drug).await
    }

    /// Get defined drugs from edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_drugs(&self) -> Result<Vec<Drug>, Error> {
        self.get::<DrugsEndpoint>("/drugs", None).await
    }

    /// Get a drug by name.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_drug_with_name(&self, name: String) -> Result<Option<Drug>, Error> {
        let filters = Filters::new(HashMap::from([(String::from("name"), name)]));
        let drugs = self.get::<DrugsEndpoint>("/drugs", Some(filters)).await?;
        Ok(drugs.first().cloned())
    }
}

#[cfg(test)]
mod tests {
    use edihkal_tracing::test_helpers::lazy_tracing;
    use entity::drug;
    use entity::NewDrug;
    use wiremock::{
        matchers::{body_json, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use crate::Client;

    #[tokio::test]
    async fn define_drug() {
        // Arrange
        lazy_tracing();

        let mock_server = MockServer::start().await;
        let mock_uri = mock_server.uri();
        let client = Client::new(mock_uri);

        let new_drug = NewDrug::new("ketamine");
        let response_body = drug::Model::from(new_drug.clone());

        Mock::given(method("POST"))
            .and(path("/drugs"))
            .and(body_json(&new_drug))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        client.define_drug(new_drug).await.expect("Failed to define new drug");

        // Assert
    }
}
