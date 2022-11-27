// Re-export drug Model as Drug for client-side apps.
use entity::drug::Model as Drug;
use entity::drug::NewDrug;

use crate::{
    edihkal::{Client, Endpoint},
    errors::Error,
};

pub(crate) struct DrugEndpoint;

impl Endpoint for DrugEndpoint {
    type NewModel = NewDrug;
    type Model = Drug;
}

impl Client {
    /// Define a drug in edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn define_drug(&self, drug: NewDrug) -> Result<Drug, Error> {
        self.post::<DrugEndpoint>("/drugs", drug).await
    }

    /// Get defind drugs from edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_drugs(&self) -> Result<Vec<Drug>, Error> {
        self.get::<DrugEndpoint>("/drugs").await
    }
}

#[cfg(test)]
mod tests {
    use edihkal_tracing::test_helpers::lazy_tracing;
    use entity::drug::{self, NewDrug};
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
