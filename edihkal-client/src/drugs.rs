use edihkal_core::drugs::Drug;

use crate::{
    edihkal::{Client, Endpoint, Response},
    errors::Error,
};
struct DrugEndpoint;

impl Endpoint for DrugEndpoint {
    type Output = Drug;
}

impl Client<'_> {
    /// Define a drug in edihkal.
    pub fn create_drug(&self, name: &str) -> Result<Response<Drug>, Error> {
        let name = name.to_string();
        let path = "/drugs";
        let payload = Drug { name };
        match serde_json::to_value(payload) {
            Ok(json) => self.post::<DrugEndpoint>(path, json),
            Err(_) => Err(Error::Deserialization(String::from(
                "Cannot serialize create_drug payload to JSON",
            ))),
        }
    }
}
