use crate::{
    edihkal::{Client, Payloads},
    entity::entry,
    errors::Error,
};

pub(crate) struct NewEntryEndpoint;

impl Payloads for NewEntryEndpoint {
    type Request = entry::NewModel;
    type Response = entry::Model;
}

impl Client {
    /// Record a new journal entry in edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn new_entry(&self, entry: entry::NewModel) -> Result<entry::Model, Error> {
        self.post::<NewEntryEndpoint>("/entries", entry).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use edihkal_tracing::test_helpers::lazy_tracing;
    use entity::{entry, Uuid};
    use wiremock::{
        matchers::{body_json, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use crate::Client;

    #[tokio::test]
    async fn new_entry() {
        // Arrange
        lazy_tracing();

        let mock_server = MockServer::start().await;
        let mock_uri = mock_server.uri();
        let client = Client::new(mock_uri);

        let dose = 3;
        let drug_id = Uuid::new_v4();
        let time = Local::now().naive_local();
        // TODO: Units of measurement!
        let new_entry = entry::NewModel::new(drug_id, time, dose);
        let response_body = entry::Model {
            id: Uuid::new_v4(),
            dose,
            drug_id,
            time,
        };

        Mock::given(method("POST"))
            .and(path("/entries"))
            .and(body_json(&new_entry))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        client.new_entry(new_entry).await.expect("Failed to record new entry");

        // Assert
    }
}
