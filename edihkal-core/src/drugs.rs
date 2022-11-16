use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDrug {
    pub name: String,
}

impl NewDrug {
    pub fn new<S: Into<String>>(name: S) -> NewDrug {
        NewDrug { name: name.into() }
    }
}
