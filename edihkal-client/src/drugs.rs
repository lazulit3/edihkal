// Re-export drug Model as Drug for client-side apps.
use entity::drug::Model as Drug;
use entity::drug::NewDrug;

use crate::{
    edihkal::{Client, Endpoint, Response},
    errors::Error,
};

struct DrugEndpoint;

impl Endpoint for DrugEndpoint {
    type Output = Drug;
}

impl Client {
    /// Define a drug in edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn define_drug(&self, drug: &NewDrug) -> Result<Response<Drug>, Error> {
        let path = "/drugs";
        match serde_json::to_value(drug) {
            Ok(json) => self.post::<DrugEndpoint>(path, json),
            Err(_) => Err(Error::Deserialization(String::from(
                "Cannot serialize define_drug payload to JSON",
            ))),
        }
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
        let response_body = drug::Model::new("ketamine");

        Mock::given(method("POST"))
            .and(path("/drugs"))
            .and(body_json(&new_drug))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        client.define_drug(&new_drug).expect("Failed to define new drug");

        // Assert
    }
}
