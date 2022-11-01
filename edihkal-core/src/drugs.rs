use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Drug {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DrugInputs {
    pub name: String,
}

impl DrugInputs {
    pub fn new<S>(name: S) -> DrugInputs
    where
        S: Into<String>,
    {
        DrugInputs { name: name.into() }
    }
}
