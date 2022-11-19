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
