use entity::entry::Model as Entry;
use entity::NewEntry;

use crate::{
    edihkal::{Client, Payloads},
    errors::Error,
};

pub(crate) struct NewEntryEndpoint;

impl Payloads for NewEntryEndpoint {
    type Request = NewEntry;
    type Response = Entry;
}

impl Client {
    /// Record a new journal entry in edihkal.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn new_entry(&self, entry: NewEntry) -> Result<Entry, Error> {
        self.post::<NewEntryEndpoint>("/entries", entry).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use edihkal_tracing::test_helpers::lazy_tracing;
    use entity::{entry, NewEntry, Uuid};
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
        let now = Local::now().naive_local();
        // TODO: NewEntry should take a reference to a drug, not just the ID.
        // TODO: Units of measurement!
        let new_entry = NewEntry::new(dose, drug_id, now);

        let response_body = entry::Model::from(new_entry.clone());
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
